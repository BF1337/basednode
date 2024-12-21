use frame_support::{
    parameter_types,
    traits::{Everything, StorageMapShim, Hooks},
    weights
};
use sp_core::H256;
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup, ConstU32},
    testing::Header,
    DispatchError
};
use sp_core::U256;
use frame_system as system;
use frame_system::{limits, EnsureNever};
use sp_consensus_aura::sr25519::AuthorityId as AuraId;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
		System: frame_system,
        Balances: pallet_balances,
		AdminUtils: pallet_admin_utils,
		BasedNode: pallet_basednode::{Pallet, Call, Storage, Event<T>}
	}
);

#[allow(dead_code)]
pub type BasednodeCall = pallet_basednode::Call<Test>;

#[allow(dead_code)]
pub type BasednodeEvent = pallet_basednode::Event<Test>;

#[allow(dead_code)]
pub type BalanceCall = pallet_balances::Call<Test>;

#[allow(dead_code)]
pub type TestRuntimeCall = frame_system::Call<Test>;

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const SS58Prefix: u8 = 42;
}

#[allow(dead_code)]
pub type AccountId = U256;

// The address format for describing accounts.
#[allow(dead_code)]
pub type Address = AccountId;

// Balance of an account.
#[allow(dead_code)]
pub type Balance = u64;

// An index to a block.
#[allow(dead_code)]
pub type BlockNumber = u64;

parameter_types! {
    pub const InitialMinAllowedWeights: u16 = 0;
    pub const InitialEmissionValue: u16 = 0;
    pub const InitialMaxWeightsLimit: u16 = u16::MAX;
    pub BlockWeights: limits::BlockWeights = limits::BlockWeights::simple_max(weights::Weight::from_ref_time(1024));
    pub const ExistentialDeposit: Balance = 1;
    pub const TransactionByteFee: Balance = 100;
    pub const SDebug:u64 = 1;
    pub const InitialRho: u16 = 30;
    pub const InitialKappa: u16 = 32_767;
    pub const InitialTempo: u16 = 0;
    pub const SelfOwnership: u64 = 2;
    pub const InitialImmunityPeriod: u16 = 2;
    pub const InitialMaxAllowedUids: u16 = 2;
    pub const InitialBondsMovingAverage: u64 = 900_000;
    pub const InitialStakePruningMin: u16 = 0;
    pub const InitialFoundationDistribution: u64 = 0;
    pub const InitialDefaultTake: u16 = 26_214; // 40% honest number.
    pub const InitialWeightsVersionKey: u16 = 0;
    pub const InitialServingRateLimit: u64 = 0; // No limit.
    pub const InitialTxRateLimit: u64 = 0; // Disable rate limit for testing
    pub const InitialBurn: u64 = 0;
    pub const InitialMinBurn: u64 = 0;
    pub const InitialMaxBurn: u64 = 1_000_000_000;
    pub const InitialValidatorPruneLen: u64 = 0;
    pub const InitialScalingLawPower: u16 = 50;
    pub const InitialMaxAllowedValidators: u16 = 100;
    pub const InitialIssuance: u64 = 0;
    pub const InitialDifficulty: u64 = 10000;
    pub const InitialActivityCutoff: u16 = 5000;
    pub const InitialAdjustmentInterval: u16 = 100;
    pub const InitialAdjustmentAlpha: u64 = 0; // no weight to previous value.
    pub const InitialMaxRegistrationsPerBlock: u16 = 3;
    pub const InitialTargetRegistrationsPerInterval: u16 = 2;
    pub const InitialPruningScore : u16 = u16::MAX;
    pub const InitialRegistrationRequirement: u16 = u16::MAX; // Top 100%
    pub const InitialMinDifficulty: u64 = 1;
    pub const InitialMaxDifficulty: u64 = u64::MAX;
    pub const InitialRAORecycledForRegistration: u64 = 0;
    pub const InitialSenateRequiredStakePercentage: u64 = 2; // 2 percent of total stake
    pub const InitialNetworkImmunityPeriod: u64 = 7200 * 7;
    pub const InitialNetworkMinAllowedUids: u16 = 128;
    pub const InitialNetworkMinLockCost: u64 = 100_000_000_000;
    pub const InitialBrainOwnerCut: u16 = 0; // 0%. 100% of rewards go to validators + miners.
    pub const InitialNetworkLockReductionInterval: u64 = 2; // 2 blocks.
    pub const InitialBrainLimit: u16 = 10; // Max 10 brains.
    pub const InitialNetworkRateLimit: u64 = 0;
	pub const InitialBrainOwnerByTokenBalanceCut: u128 = 10;
}

impl pallet_basednode::Config for Test
{
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type InitialIssuance = InitialIssuance;
    type SudoRuntimeCall = TestRuntimeCall;
    type CouncilOrigin = EnsureNever<AccountId>;
    type SenateMembers = ();
    type TriumvirateInterface = ();

    type InitialMinAllowedWeights = InitialMinAllowedWeights;
    type InitialEmissionValue = InitialEmissionValue;
    type InitialMaxWeightsLimit = InitialMaxWeightsLimit;
    type InitialTempo = InitialTempo;
    type InitialDifficulty = InitialDifficulty;
    type InitialAdjustmentInterval = InitialAdjustmentInterval;
    type InitialAdjustmentAlpha = InitialAdjustmentAlpha;
    type InitialTargetRegistrationsPerInterval = InitialTargetRegistrationsPerInterval;
    type InitialRho = InitialRho;
    type InitialKappa = InitialKappa;
    type InitialMaxAllowedUids = InitialMaxAllowedUids;
    type InitialValidatorPruneLen = InitialValidatorPruneLen;
    type InitialScalingLawPower = InitialScalingLawPower;
    type InitialImmunityPeriod = InitialImmunityPeriod;
    type InitialActivityCutoff = InitialActivityCutoff;
    type InitialMaxRegistrationsPerBlock = InitialMaxRegistrationsPerBlock;
    type InitialPruningScore = InitialPruningScore;
    type InitialBondsMovingAverage = InitialBondsMovingAverage;
    type InitialMaxAllowedValidators = InitialMaxAllowedValidators;
    type InitialDefaultTake = InitialDefaultTake;
    type InitialWeightsVersionKey = InitialWeightsVersionKey;
    type InitialMaxDifficulty = InitialMaxDifficulty;
    type InitialMinDifficulty = InitialMinDifficulty;
    type InitialServingRateLimit = InitialServingRateLimit;
    type InitialTxRateLimit = InitialTxRateLimit;
    type InitialBurn = InitialBurn;
    type InitialMaxBurn = InitialMaxBurn;
    type InitialMinBurn = InitialMinBurn;
    type InitialRAORecycledForRegistration = InitialRAORecycledForRegistration;
    type InitialSenateRequiredStakePercentage = InitialSenateRequiredStakePercentage;
    type InitialNetworkImmunityPeriod = InitialNetworkImmunityPeriod;
    type InitialNetworkMinAllowedUids = InitialNetworkMinAllowedUids;
    type InitialNetworkMinLockCost = InitialNetworkMinLockCost;
    type InitialBrainOwnerCut = InitialBrainOwnerCut;
    type InitialNetworkLockReductionInterval = InitialNetworkLockReductionInterval;
    type InitialBrainLimit = InitialBrainLimit;
    type InitialNetworkRateLimit = InitialNetworkRateLimit;
	type InitialBrainOwnerByTokenBalanceCut = InitialBrainOwnerByTokenBalanceCut;
}

impl system::Config for Test {
    type BaseCallFilter = Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = U256;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = BlockHashCount;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = SS58Prefix;
    type OnSetCode = ();
    type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_balances::Config for Test {
    type Balance = Balance;
    type RuntimeEvent = RuntimeEvent;
    type DustRemoval = ();
    type ExistentialDeposit = ();
    type AccountStore = StorageMapShim<
        pallet_balances::Account<Test>,
        frame_system::Provider<Test>,
        AccountId,
        pallet_balances::AccountData<Balance>,
    >;
    type MaxLocks = ();
    type WeightInfo = ();
    type MaxReserves = ();
    type ReserveIdentifier = ();
}

pub struct BasednodeIntrf;

impl pallet_admin_utils::BasednodeInterface<AccountId, Balance, RuntimeOrigin> for BasednodeIntrf
{
    fn set_default_take(default_take: u16)
    {
        BasedNode::set_default_take(default_take);
    }

	fn set_tx_rate_limit(rate_limit: u64)
    {
        BasedNode::set_tx_rate_limit(rate_limit);
    }

	fn set_serving_rate_limit(netuid: u16, rate_limit: u64)
    {
        BasedNode::set_serving_rate_limit(netuid, rate_limit);
    }

	fn set_max_burn(netuid: u16, max_burn: u64)
    {
        BasedNode::set_max_burn(netuid, max_burn);
    }

	fn set_min_burn(netuid: u16, min_burn: u64)
    {
        BasedNode::set_min_burn(netuid, min_burn);
    }

	fn set_burn(netuid: u16, burn: u64)
    {
        BasedNode::set_burn(netuid, burn);
    }

	fn set_max_difficulty(netuid: u16, max_diff: u64)
    {
        BasedNode::set_max_difficulty(netuid, max_diff);
    }

	fn set_min_difficulty(netuid: u16, min_diff: u64)
    {
        BasedNode::set_min_difficulty(netuid, min_diff);
    }

	fn set_difficulty(netuid: u16, diff: u64)
    {
        BasedNode::set_difficulty(netuid, diff);
    }

	fn set_weights_rate_limit(netuid: u16, rate_limit: u64)
    {
        BasedNode::set_weights_set_rate_limit(netuid, rate_limit);
    }

	fn set_weights_version_key(netuid: u16, version: u64)
    {
        BasedNode::set_weights_version_key(netuid, version);
    }

	fn set_bonds_moving_average(netuid: u16, moving_average: u64)
    {
        BasedNode::set_bonds_moving_average(netuid, moving_average);
    }

	fn set_max_allowed_validators(netuid: u16, max_validators: u16)
    {
        BasedNode::set_max_allowed_validators(netuid, max_validators);
    }

	fn get_root_netuid() -> u16
    {
        return BasedNode::get_root_netuid();
    }

	fn if_brain_exist(netuid: u16) -> bool
    {
        return BasedNode::if_brain_exist(netuid);
    }

	fn create_account_if_non_existent(personalkey: &AccountId, computekey: &AccountId)
    {
        return BasedNode::create_account_if_non_existent(personalkey, computekey);
    }

	fn personalkey_owns_computekey(personalkey: &AccountId, computekey: &AccountId) -> bool
    {
        return BasedNode::personalkey_owns_computekey(personalkey, computekey);
    }

	fn increase_stake_on_personalkey_computekey_account(personalkey: &AccountId, computekey: &AccountId, increment: u64)
    {
        BasedNode::increase_stake_on_personalkey_computekey_account(personalkey, computekey, increment);
    }

	fn u64_to_balance(input: u64) -> Option<Balance>
    {
        return BasedNode::u64_to_balance(input);
    }

	fn add_balance_to_personalkey_account(personalkey: &AccountId, amount: Balance)
    {
        BasedNode::add_balance_to_personalkey_account(personalkey, amount);
    }

	fn get_current_block_as_u64() -> u64
    {
        return BasedNode::get_current_block_as_u64();
    }

	fn get_brain_n(netuid: u16) -> u16
    {
        return BasedNode::get_brain_n(netuid);
    }

	fn get_max_allowed_uids(netuid: u16) -> u16
    {
        return BasedNode::get_max_allowed_uids(netuid);
    }

	fn append_agent(netuid: u16, new_computekey: &AccountId, block_number: u64)
    {
        return BasedNode::append_agent(netuid, new_computekey, block_number);
    }

	fn get_agent_to_prune(netuid: u16) -> u16
    {
        return BasedNode::get_agent_to_prune(netuid);
    }

	fn replace_agent(netuid: u16, uid_to_replace: u16, new_computekey: &AccountId, block_number: u64)
    {
        BasedNode::replace_agent(netuid, uid_to_replace, new_computekey, block_number);
    }

	fn set_total_issuance(total_issuance: u64)
    {
        BasedNode::set_total_issuance(total_issuance);
    }

	fn set_network_immunity_period(net_immunity_period: u64)
    {
        BasedNode::set_network_immunity_period(net_immunity_period);
    }

	fn set_network_min_lock(net_min_lock: u128)
    {
        BasedNode::set_network_min_lock(net_min_lock);
    }

    fn set_brain_limit(limit: u16)
    {
        BasedNode::set_max_brains(limit);
    }

    fn set_lock_reduction_interval(interval: u64)
    {
        BasedNode::set_lock_reduction_interval(interval);
    }

    fn set_tempo(netuid: u16, tempo: u16)
    {
        BasedNode::set_tempo(netuid, tempo);
    }

    fn set_brain_owner_cut(brain_owner_cut: u16)
    {
        BasedNode::set_brain_owner_cut(brain_owner_cut);
    }

    fn set_network_rate_limit(limit: u64)
    {
        BasedNode::set_network_rate_limit(limit);
    }

    fn set_max_registrations_per_block(netuid: u16, max_registrations_per_block: u16)
    {
        BasedNode::set_max_registrations_per_block(netuid, max_registrations_per_block);
    }

    fn set_adjustment_alpha(netuid: u16, adjustment_alpha: u64)
    {
        BasedNode::set_adjustment_alpha(netuid, adjustment_alpha);
    }

    fn set_target_registrations_per_interval(netuid: u16, target_registrations_per_interval: u16)
    {
        BasedNode::set_target_registrations_per_interval(netuid, target_registrations_per_interval);
    }

    fn set_network_pow_registration_allowed(netuid: u16, registration_allowed: bool)
    {
        BasedNode::set_network_pow_registration_allowed(netuid, registration_allowed);
    }

    fn set_network_registration_allowed(netuid: u16, registration_allowed: bool)
    {
        BasedNode::set_network_pow_registration_allowed(netuid, registration_allowed);
    }

    fn set_activity_cutoff(netuid: u16, activity_cutoff: u16)
    {
        BasedNode::set_activity_cutoff(netuid, activity_cutoff);
    }

    fn ensure_brain_owner_or_root(o: RuntimeOrigin, netuid: u16) -> Result<(), DispatchError>
    {
        return BasedNode::ensure_brain_owner_or_root(o, netuid);
    }

    fn set_rho(netuid: u16, rho: u16)
    {
        BasedNode::set_rho(netuid, rho);
    }

    fn set_kappa(netuid: u16, kappa: u16)
    {
        BasedNode::set_kappa(netuid, kappa);
    }

    fn set_max_allowed_uids(netuid: u16, max_allowed: u16)
    {
        BasedNode::set_max_allowed_uids(netuid, max_allowed);
    }

    fn set_min_allowed_weights(netuid: u16, min_allowed_weights: u16)
    {
        BasedNode::set_min_allowed_weights(netuid, min_allowed_weights);
    }

    fn set_immunity_period(netuid: u16, immunity_period: u16)
    {
        BasedNode::set_immunity_period(netuid, immunity_period);
    }

    fn set_max_weight_limit(netuid: u16, max_weight_limit: u16)
    {
        BasedNode::set_max_weight_limit(netuid, max_weight_limit);
    }

    fn set_scaling_law_power(netuid: u16, scaling_law_power: u16)
    {
        BasedNode::set_scaling_law_power(netuid, scaling_law_power);
    }

    fn set_validator_prune_len(netuid: u16, validator_prune_len: u64)
    {
        BasedNode::set_validator_prune_len(netuid, validator_prune_len);
    }

    fn set_adjustment_interval(netuid: u16, adjustment_interval: u16)
    {
        BasedNode::set_adjustment_interval(netuid, adjustment_interval);
    }

    fn set_weights_set_rate_limit(netuid: u16, weights_set_rate_limit: u64)
    {
        BasedNode::set_weights_set_rate_limit(netuid, weights_set_rate_limit);
    }

    fn set_rao_recycled(netuid: u16, rao_recycled: u64)
    {
        BasedNode::set_rao_recycled(netuid, rao_recycled);
    }

    fn is_computekey_registered_on_network(netuid: u16, computekey: &AccountId) -> bool
    {
        return BasedNode::is_computekey_registered_on_network(netuid, computekey);
    }

    fn init_new_network(netuid: u16, tempo: u16)
    {
        BasedNode::init_new_network(netuid, tempo);
    }
}

impl pallet_admin_utils::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type AuthorityId = AuraId;
    type MaxAuthorities = ConstU32<32>;
    type Aura = ();
    type Balance = Balance;
    type Basednode = BasednodeIntrf;
    type WeightInfo = /*pallet_admin_utils::weights::SubstrateWeight<Test>*/();
}

#[allow(dead_code)]
pub fn new_test_ext() -> sp_io::TestExternalities
{
    sp_tracing::try_init_simple();
    frame_system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap()
        .into()
}

#[allow(dead_code)]
pub(crate) fn run_to_block(n: u64) {
    while System::block_number() < n {
        BasedNode::on_finalize(System::block_number());
        System::on_finalize(System::block_number());
        System::set_block_number(System::block_number() + 1);
        System::on_initialize(System::block_number());
        BasedNode::on_initialize(System::block_number());
    }
}
