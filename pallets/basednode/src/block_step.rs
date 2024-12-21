use super::*;
use frame_support::inherent::Vec;
use frame_support::storage::IterableStorageDoubleMap;
use frame_support::storage::IterableStorageMap;
use substrate_fixed::types::I110F18;
use substrate_fixed::types::I64F64;
use substrate_fixed::types::I96F32;

// 1M of total tokens with 10^18 precision,
const BRAIN_ERC20_TOTAL_SUPPLY: i128 = 1_000_000_000_000_000_000_000_000;

impl<T: Config> Pallet<T> {
    /// Executes the necessary operations for each block.
    pub fn block_step() -> Result<(), &'static str> {
        let block_number: u64 = Self::get_current_block_as_u64();
        log::debug!("block_step for block: {:?} ", block_number);
        // --- 1. Adjust difficulties.
        Self::adjust_registration_terms_for_networks();
        // --- 2. Calculate per-brain emissions
        match Self::root_epoch(block_number) {
            Ok(_) => (),
            Err(e) => {
                log::error!("Error while running root epoch: {:?}", e);
            }
        }
        // --- 3. Drains emission tuples ( computekey, amount ).
        Self::drain_emission(block_number);
        // --- 4. Generates emission tuples from epoch functions.
        Self::generate_emission(block_number);
        // Return ok.
        Ok(())
    }

    // Helper function which returns the number of blocks remaining before we will run the epoch on this
    // network. Networks run their epoch when (block_number + netuid + 1 ) % (tempo + 1) = 0
    //
    pub fn blocks_until_next_epoch(netuid: u16, tempo: u16, block_number: u64) -> u64 {
        // tempo | netuid | # first epoch block
        //   1        0               0
        //   1        1               1
        //   2        0               1
        //   2        1               0
        //   100      0              99
        //   100      1              98
        // Special case: tempo = 0, the network never runs.
        if tempo == 0 {
            return 1000;
        }
        return tempo as u64 - (block_number + netuid as u64 + 1) % (tempo as u64 + 1);
    }

    // Helper function returns the number of tuples to drain on a particular step based on
    // the remaining tuples to sink and the block number
    //
    pub fn tuples_to_drain_this_block(
        netuid: u16,
        tempo: u16,
        block_number: u64,
        n_remaining: usize,
    ) -> usize {
        let blocks_until_epoch: u64 = Self::blocks_until_next_epoch(netuid, tempo, block_number);
        if blocks_until_epoch / 2 == 0 {
            return n_remaining;
        } // drain all.
        if tempo / 2 == 0 {
            return n_remaining;
        } // drain all
        if n_remaining == 0 {
            return 0;
        } // nothing to drain at all.
          // Else return enough tuples to drain all within half the epoch length.
        let to_sink_via_tempo: usize = n_remaining / (tempo as usize / 2);
        let to_sink_via_blocks_until_epoch: usize = n_remaining / (blocks_until_epoch as usize / 2);
        if to_sink_via_tempo > to_sink_via_blocks_until_epoch {
            return to_sink_via_tempo;
        } else {
            return to_sink_via_blocks_until_epoch;
        }
    }

    pub fn has_loaded_emission_tuples(netuid: u16) -> bool {
        LoadedEmission::<T>::contains_key(netuid)
    }
    pub fn get_loaded_emission_tuples(netuid: u16) -> Vec<(T::AccountId, u64, u64)> {
        LoadedEmission::<T>::get(netuid).unwrap()
    }

    // Reads from the loaded emission storage which contains lists of pending emission tuples ( computekey, amount )
    // and distributes small chunks of them at a time.
    //
    pub fn drain_emission(_: u64) {
        // --- 1. We iterate across each network.
        for (netuid, _) in <Tempo<T> as IterableStorageMap<u16, u16>>::iter() {
            log::trace!("drain_emission netuid: {:?}, LoadedEmissions::<T>: {:?}", netuid, LoadedEmission::<T>::get(netuid));
            if !Self::has_loaded_emission_tuples(netuid) {
                continue;
            } // There are no tuples to emit.
            let tuples_to_drain: Vec<(T::AccountId, u64, u64)> =
                Self::get_loaded_emission_tuples(netuid);
            log::trace!("drain_emission tuples_to_drain: {:?}", tuples_to_drain);
            let mut total_emitted: u64 = 0;
            for (computekey, server_amount, validator_amount) in tuples_to_drain.iter() {
                Self::emit_inflation_through_computekey_account(
                    &computekey,
                    *server_amount,
                    *validator_amount,
                );
                total_emitted += *server_amount + *validator_amount;
            }
            LoadedEmission::<T>::remove(netuid);
            TotalIssuance::<T>::put(TotalIssuance::<T>::get().saturating_add(total_emitted.into()));
        }
    }

    // Iterates through networks queues more emission onto their pending storage.
    // If a network has no blocks left until tempo, we run the epoch function and generate
    // more token emission tuples for later draining onto accounts.
    //
    pub fn generate_emission(block_number: u64) {
        // --- 1. Iterate across each network and add pending emission into stash.
        for (netuid, tempo) in <Tempo<T> as IterableStorageMap<u16, u16>>::iter() {
            // FIXME should root emissions be burned?
            // Skip the root network.
            if netuid == Self::get_root_netuid() {
                // Root emission is burned.
                continue;
            }

            // --- 2. Queue the emission due to this network.
            let new_queued_emission = Self::get_brain_emission_value(netuid);
            log::debug!(
                "generate_emission for netuid: {:?} with tempo: {:?} and emission: {:?}",
                netuid,
                tempo,
                new_queued_emission,
            );

            let brain_has_owner = BrainOwner::<T>::contains_key(netuid);
            let mut remaining = I96F32::from_num(new_queued_emission);
            if brain_has_owner {
                let owner = BrainOwner::<T>::get(netuid);

                // TODO(emissions) check fixed types length
                let contract_activated = BrainContract::<T>::contains_key(netuid);
                if contract_activated {
                    let contract_address = BrainContract::<T>::get(netuid);
                    log::debug!("contract is activated for brain = {:?}, addres = {:?}", netuid, contract_address);
                    let mut total_cut = I96F32::from(0);
                    let mut cuts = vec![];

                    // (emissions) iter over all 2nd keys (token balance owner) of first key (which is brain owner),
                    // value / total tokens is percentage of the cut, returns list of (AccountId,
                    // value)
                    for (token_owner, balance) in
                        BrainOwnerByTokenBalanceCut::<T>::iter_prefix(contract_address)
                    {
                        if balance < (BRAIN_ERC20_TOTAL_SUPPLY / 100 * 2) as u128 {
                            continue;
                        }
                        let balance_ratio = I96F32::from_num(balance)
                            .saturating_div(I96F32::from_num(BRAIN_ERC20_TOTAL_SUPPLY));
                        log::debug!("token_owner = {:?}, balance = {:?}, balance_ratio = {:?}", token_owner, balance, balance_ratio);

                        let cut = remaining
                            .saturating_mul(I96F32::from_num(Self::get_brain_owner_cut()))
                            .saturating_div(I96F32::from_num(u16::MAX))
                            .saturating_mul(balance_ratio);
                        total_cut = total_cut.saturating_add(cut);
                        remaining.saturating_sub(cut);
                        cuts.push((token_owner, cut));
                    }

                    log::debug!("distributing cuts: {:?}", cuts);

                    // TODO(emissions) see mem usage in test and possibly move to upper loop and do
                    // not allocate cuts Vec
                    for (owner, cut) in cuts {
                        Self::add_balance_to_personalkey_account(
                            &owner,
                            Self::u64_to_balance(cut.to_num::<u64>()).unwrap(),
                        );
                    }

                    TotalIssuance::<T>::put(
                        TotalIssuance::<T>::get().saturating_add(total_cut.to_num::<u128>()),
                    );
                } else {
                    log::trace!("generate_emissions, contract_activated = false, remaining = {:?}", remaining);
                    let cut = remaining
                        .saturating_mul(I96F32::from_num(Self::get_brain_owner_cut()))
                        .saturating_div(I96F32::from_num(u16::MAX));

                    remaining = remaining.saturating_sub(cut);
                    log::trace!("generate_emissions, contract_activated = false, cut = {:?} remaining = {:?}", cut, remaining);

                    // iter over all tuples, add to PK and subtract from the remaining pull
                    Self::add_balance_to_personalkey_account(
                        &Self::get_brain_owner(netuid),
                        Self::u64_to_balance(cut.to_num::<u64>()).unwrap(),
                    );
                    // TODO(emissions) sum of all cuts
                    TotalIssuance::<T>::put(
                        TotalIssuance::<T>::get().saturating_add(cut.to_num::<u128>()),
                    );
                }
            }
            // --- 5. Add remaining amount to the network's pending emission.
            PendingEmission::<T>::mutate(netuid, |queued| *queued += remaining.to_num::<u64>());
            log::debug!(
                "netuid_i: {:?} queued_emission: +{:?} , PendingEmission::<T> = {:?}",
                netuid,
                remaining,
                PendingEmission::<T>::get(netuid)
            );

            // --- 6. Check to see if this network has reached tempo.
            if Self::blocks_until_next_epoch(netuid, tempo, block_number) != 0 {
                // --- 3.1 No epoch, increase blocks since last step and continue,
                Self::set_blocks_since_last_step(
                    netuid,
                    Self::get_blocks_since_last_step(netuid) + 1,
                );
                continue;
            }

            // --- 7 This network is at tempo and we are running its epoch.
            // First drain the queued emission.
            let emission_to_drain: u64 = PendingEmission::<T>::get(netuid);
            PendingEmission::<T>::insert(netuid, 0);

            // --- 8. Run the epoch mechanism and return emission tuples for computekeys in the network.
            let emission_tuples_this_block: Vec<(T::AccountId, u64, u64)> =
                Self::epoch(netuid, emission_to_drain);
            log::debug!(
                "netuid_i: {:?} emission_to_drain: {:?} emission_tuples_this_block: {:?}",
                netuid,
                emission_to_drain,
                emission_tuples_this_block,
            );

            // --- 9. Check that the emission does not exceed the allowed total.
            let emission_sum: u128 = emission_tuples_this_block
                .iter()
                .map(|(_account_id, ve, se)| *ve as u128 + *se as u128)
                .sum();
            if emission_sum > emission_to_drain as u128 {
                continue;
            } // Saftey check.

            // --- 10. Sink the emission tuples onto the already loaded.
            let mut concat_emission_tuples: Vec<(T::AccountId, u64, u64)> =
                emission_tuples_this_block.clone();
            if Self::has_loaded_emission_tuples(netuid) {
                // 10.a We already have loaded emission tuples, so we concat the new ones.
                let mut current_emission_tuples: Vec<(T::AccountId, u64, u64)> =
                    Self::get_loaded_emission_tuples(netuid);
                concat_emission_tuples.append(&mut current_emission_tuples);
            }
            LoadedEmission::<T>::insert(netuid, concat_emission_tuples);

            // --- 11 Set counters.
            Self::set_blocks_since_last_step(netuid, 0);
            Self::set_last_mechanism_step_block(netuid, block_number);
        }
    }
    // Distributes token inflation through the computekey based on emission. The call ensures that the inflation
    // is distributed onto the accounts in proportion of the stake delegated minus the take. This function
    // is called after an epoch to distribute the newly minted stake according to delegation.
    //
    pub fn emit_inflation_through_computekey_account(
        computekey: &T::AccountId,
        server_emission: u64,
        validator_emission: u64,
    ) {
        log::debug!("emit_inflation_through_computekey_account ck: {:?}, server_emission: {:?}, validator_emission: {:?}",
            computekey, server_emission, validator_emission
            );
        // --- 1. Check if the computekey is a delegate. If not, we simply pass the stake through to the
        // personalkey - computekey account as normal.
        if !Self::computekey_is_delegate(computekey) {
            Self::increase_stake_on_computekey_account(
                &computekey,
                server_emission + validator_emission,
            );
            return;
        }
        // Then this is a delegate, we distribute validator_emission, then server_emission.

        // --- 2. The computekey is a delegate. We first distribute a proportion of the validator_emission to the computekey
        // directly as a function of its 'take'
        let total_computekey_stake: u64 = Self::get_total_stake_for_computekey(computekey);
        let delegate_take: u64 =
            Self::calculate_delegate_proportional_take(computekey, validator_emission);
        let validator_emission_minus_take: u64 = validator_emission - delegate_take;
        let mut remaining_validator_emission: u64 = validator_emission_minus_take;

        // 3. -- The remaining emission goes to the owners in proportion to the stake delegated.
        for (owning_personalkey_i, stake_i) in
            <Stake<T> as IterableStorageDoubleMap<T::AccountId, T::AccountId, u64>>::iter_prefix(
                computekey,
            )
        {
            // --- 4. The emission proportion is remaining_emission * ( stake / total_stake ).
            let stake_proportion: u64 = Self::calculate_stake_proportional_emission(
                stake_i,
                total_computekey_stake,
                validator_emission_minus_take,
            );
            Self::increase_stake_on_personalkey_computekey_account(
                &owning_personalkey_i,
                &computekey,
                stake_proportion,
            );
            log::debug!(
                "owning_personalkey_i: {:?} computekey: {:?} emission: +{:?} ",
                owning_personalkey_i,
                computekey,
                stake_proportion
            );
            remaining_validator_emission -= stake_proportion;
        }

        // --- 5. Last increase final account balance of delegate after 4, since 5 will change the stake proportion of
        // the delegate and effect calculation in 4.
        Self::increase_stake_on_computekey_account(
            &computekey,
            delegate_take + remaining_validator_emission,
        );
        log::debug!(
            "delkey: {:?} delegate_take: +{:?} ",
            computekey,
            delegate_take
        );
        // Also emit the server_emission to the computekey
        // The server emission is distributed in-full to the delegate owner.
        // We do this after 4. for the same reason as above.
        Self::increase_stake_on_computekey_account(&computekey, server_emission);
    }

    // Increases the stake on the cold - hot pairing by increment while also incrementing other counters.
    // This function should be called rather than set_stake under account.
    //
    pub fn block_step_increase_stake_on_personalkey_computekey_account(
        personalkey: &T::AccountId,
        computekey: &T::AccountId,
        increment: u64,
    ) {
        TotalPersonalkeyStake::<T>::mutate(personalkey, |old| old.saturating_add(increment));
        TotalComputekeyStake::<T>::insert(
            computekey,
            TotalComputekeyStake::<T>::get(computekey).saturating_add(increment),
        );
        Stake::<T>::insert(
            computekey,
            personalkey,
            Stake::<T>::get(computekey, personalkey).saturating_add(increment),
        );
        TotalStake::<T>::put(TotalStake::<T>::get().saturating_add(increment));
        TotalIssuance::<T>::put(TotalIssuance::<T>::get().saturating_add(increment.into()));
    }

    // Decreases the stake on the cold - hot pairing by the decrement while decreasing other counters.
    //
    pub fn block_step_decrease_stake_on_personalkey_computekey_account(
        personalkey: &T::AccountId,
        computekey: &T::AccountId,
        decrement: u64,
    ) {
        TotalPersonalkeyStake::<T>::mutate(personalkey, |old| old.saturating_sub(decrement));
        TotalComputekeyStake::<T>::insert(
            computekey,
            TotalComputekeyStake::<T>::get(computekey).saturating_sub(decrement),
        );
        Stake::<T>::insert(
            computekey,
            personalkey,
            Stake::<T>::get(computekey, personalkey).saturating_sub(decrement),
        );
        TotalStake::<T>::put(TotalStake::<T>::get().saturating_sub(decrement));
        TotalIssuance::<T>::put(TotalIssuance::<T>::get().saturating_sub(decrement.into()));
    }

    // Returns emission awarded to a computekey as a function of its proportion of the total stake.
    //
    pub fn calculate_stake_proportional_emission(
        stake: u64,
        total_stake: u64,
        emission: u64,
    ) -> u64 {
        if total_stake == 0 {
            return 0;
        };
        let stake_proportion: I64F64 = I64F64::from_num(stake) / I64F64::from_num(total_stake);
        let proportional_emission: I64F64 = I64F64::from_num(emission) * stake_proportion;
        return proportional_emission.to_num::<u64>();
    }

    // Returns the delegated stake 'take' assigned to this key. (If exists, otherwise 0)
    //
    pub fn calculate_delegate_proportional_take(computekey: &T::AccountId, emission: u64) -> u64 {
        if Self::computekey_is_delegate(computekey) {
            let take_proportion: I64F64 =
                I64F64::from_num(Delegates::<T>::get(computekey)) / I64F64::from_num(u16::MAX);
            let take_emission: I64F64 = take_proportion * I64F64::from_num(emission);
            return take_emission.to_num::<u64>();
        } else {
            return 0;
        }
    }

    // Adjusts the network difficulties/burns of every active network. Resetting state parameters.
    //
    pub fn adjust_registration_terms_for_networks() {
        log::debug!("adjust_registration_terms_for_networks");

        // --- 1. Iterate through each network.
        for (netuid, _) in <NetworksAdded<T> as IterableStorageMap<u16, bool>>::iter() {
            // --- 2. Pull counters for network difficulty.
            let last_adjustment_block: u64 = Self::get_last_adjustment_block(netuid);
            let adjustment_interval: u16 = Self::get_adjustment_interval(netuid);
            let current_block: u64 = Self::get_current_block_as_u64();
            log::debug!("netuid: {:?} last_adjustment_block: {:?} adjustment_interval: {:?} current_block: {:?}",
                netuid,
                last_adjustment_block,
                adjustment_interval,
                current_block
            );

            // --- 3. Check if we are at the adjustment interval for this network.
            // If so, we need to adjust the registration difficulty based on target and actual registrations.
            if (current_block - last_adjustment_block) >= adjustment_interval as u64 {
                log::debug!("interval reached.");

                // --- 4. Get the current counters for this network w.r.t burn and difficulty values.
                let current_burn: u128 = Self::get_burn_as_u64(netuid);
                let current_difficulty: u64 = Self::get_difficulty_as_u64(netuid);
                let registrations_this_interval: u16 =
                    Self::get_registrations_this_interval(netuid);
                let pow_registrations_this_interval: u16 =
                    Self::get_pow_registrations_this_interval(netuid);
                let burn_registrations_this_interval: u16 =
                    Self::get_burn_registrations_this_interval(netuid);
                let target_registrations_this_interval: u16 =
                    Self::get_target_registrations_per_interval(netuid);
                // --- 5. Adjust burn + pow
                // There are six cases to consider. A, B, C, D, E, F
                if registrations_this_interval > target_registrations_this_interval {
                    if pow_registrations_this_interval > burn_registrations_this_interval {
                        // A. There are too many registrations this interval and most of them are pow registrations
                        // this triggers an increase in the pow difficulty.
                        // pow_difficulty ++
                        Self::set_difficulty(
                            netuid,
                            Self::adjust_difficulty(
                                netuid,
                                current_difficulty,
                                registrations_this_interval,
                                target_registrations_this_interval,
                            ),
                        );
                    } else if pow_registrations_this_interval < burn_registrations_this_interval {
                        // B. There are too many registrations this interval and most of them are burn registrations
                        // this triggers an increase in the burn cost.
                        // burn_cost ++
                        Self::set_burn(
                            netuid,
                            Self::adjust_burn(
                                netuid,
                                current_burn,
                                registrations_this_interval,
                                target_registrations_this_interval,
                            ),
                        );
                    } else {
                        // F. There are too many registrations this interval and the pow and burn registrations are equal
                        // this triggers an increase in the burn cost and pow difficulty
                        // burn_cost ++
                        Self::set_burn(
                            netuid,
                            Self::adjust_burn(
                                netuid,
                                current_burn,
                                registrations_this_interval,
                                target_registrations_this_interval,
                            ),
                        );
                        // pow_difficulty ++
                        Self::set_difficulty(
                            netuid,
                            Self::adjust_difficulty(
                                netuid,
                                current_difficulty,
                                registrations_this_interval,
                                target_registrations_this_interval,
                            ),
                        );
                    }
                } else {
                    // Not enough registrations this interval.
                    if pow_registrations_this_interval > burn_registrations_this_interval {
                        // C. There are not enough registrations this interval and most of them are pow registrations
                        // this triggers a decrease in the burn cost
                        // burn_cost --
                        Self::set_burn(
                            netuid,
                            Self::adjust_burn(
                                netuid,
                                current_burn,
                                registrations_this_interval,
                                target_registrations_this_interval,
                            ),
                        );
                    } else if pow_registrations_this_interval < burn_registrations_this_interval {
                        // D. There are not enough registrations this interval and most of them are burn registrations
                        // this triggers a decrease in the pow difficulty
                        // pow_difficulty --
                        Self::set_difficulty(
                            netuid,
                            Self::adjust_difficulty(
                                netuid,
                                current_difficulty,
                                registrations_this_interval,
                                target_registrations_this_interval,
                            ),
                        );
                    } else {
                        // E. There are not enough registrations this interval and the pow and burn registrations are equal
                        // this triggers a decrease in the burn cost and pow difficulty
                        // burn_cost --
                        Self::set_burn(
                            netuid,
                            Self::adjust_burn(
                                netuid,
                                current_burn,
                                registrations_this_interval,
                                target_registrations_this_interval,
                            ),
                        );
                        // pow_difficulty --
                        Self::set_difficulty(
                            netuid,
                            Self::adjust_difficulty(
                                netuid,
                                current_difficulty,
                                registrations_this_interval,
                                target_registrations_this_interval,
                            ),
                        );
                    }
                }

                // --- 6. Drain all counters for this network for this interval.
                Self::set_last_adjustment_block(netuid, current_block);
                Self::set_registrations_this_interval(netuid, 0);
                Self::set_pow_registrations_this_interval(netuid, 0);
                Self::set_burn_registrations_this_interval(netuid, 0);
            } else {
                log::debug!("interval not reached.");
            }

            // --- 7. Drain block registrations for each network. Needed for registration rate limits.
            Self::set_registrations_this_block(netuid, 0);
        }
    }

    // Performs the difficulty adjustment by multiplying the current difficulty by the ratio ( reg_actual + reg_target / reg_target * reg_target )
    // We use I110F18 to avoid any overflows on u64. Also min_difficulty and max_difficulty bound the range.
    //
    pub fn adjust_difficulty(
        netuid: u16,
        current_difficulty: u64,
        registrations_this_interval: u16,
        target_registrations_per_interval: u16,
    ) -> u64 {
        let updated_difficulty: I110F18 = I110F18::from_num(current_difficulty)
            * I110F18::from_num(registrations_this_interval + target_registrations_per_interval)
            / I110F18::from_num(
                target_registrations_per_interval + target_registrations_per_interval,
            );
        let alpha: I110F18 =
            I110F18::from_num(Self::get_adjustment_alpha(netuid)) / I110F18::from_num(u64::MAX);
        let next_value: I110F18 = alpha * I110F18::from_num(current_difficulty)
            + (I110F18::from_num(1.0) - alpha) * updated_difficulty;
        if next_value >= I110F18::from_num(Self::get_max_difficulty(netuid)) {
            return Self::get_max_difficulty(netuid);
        } else if next_value <= I110F18::from_num(Self::get_min_difficulty(netuid)) {
            return Self::get_min_difficulty(netuid);
        } else {
            return next_value.to_num::<u64>();
        }
    }

    // Performs the burn adjustment by multiplying the current difficulty by the ratio ( reg_actual + reg_target / reg_target * reg_target )
    // We use I110F18 to avoid any overflows on u64. Also min_burn and max_burn bound the range.
    //
    pub fn adjust_burn(
        netuid: u16,
        current_burn: u128,
        registrations_this_interval: u16,
        target_registrations_per_interval: u16,
    ) -> u128 {
        let updated_burn: I110F18 = I110F18::from_num(current_burn)
            * I110F18::from_num(registrations_this_interval + target_registrations_per_interval)
            / I110F18::from_num(
                target_registrations_per_interval + target_registrations_per_interval,
            );
        let alpha: I110F18 =
            I110F18::from_num(Self::get_adjustment_alpha(netuid)) / I110F18::from_num(u64::MAX);
        let next_value: I110F18 = alpha * I110F18::from_num(current_burn)
            + (I110F18::from_num(1.0) - alpha) * updated_burn;
        if next_value >= I110F18::from_num(Self::get_max_burn_as_u64(netuid)) {
            return Self::get_max_burn_as_u64(netuid);
        } else if next_value <= I110F18::from_num(Self::get_min_burn_as_u64(netuid)) {
            return Self::get_min_burn_as_u64(netuid);
        } else {
            return next_value.to_num::<u128>();
        }
    }
}
