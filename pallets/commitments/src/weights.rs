
//! Autogenerated weights for `pallet_commitments`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-12-08, STEPS: `2`, REPEAT: `1`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `rustys-mbp.lan`, CPU: `<UNKNOWN>`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("local")`, DB CACHE: `1024`

// Executed Command:
// ./target/release/basednode
// benchmark
// pallet
// --chain=local
// --execution=wasm
// --wasm-execution=compiled
// --pallet=pallet_commitments
// --extrinsic=*
// --output=pallets/commitments/src/weights.rs
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pallet_commitments`.
pub trait WeightInfo {
	fn set_commitment() -> Weight;
}

/// Weights for `pallet_commitments` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: Commitments LastCommitment (r:1 w:1)
	/// Proof Skipped: Commitments LastCommitment (max_values: None, max_size: None, mode: Measured)
	/// Storage: Commitments CommitmentOf (r:1 w:1)
	/// Proof Skipped: Commitments CommitmentOf (max_values: None, max_size: None, mode: Measured)
	fn set_commitment() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `697`
		//  Estimated: `6344`
		// Minimum execution time: 28_000_000 picoseconds.
		Weight::from_parts(28_000_000, 6344)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	/// Storage: Commitments LastCommitment (r:1 w:1)
	/// Proof Skipped: Commitments LastCommitment (max_values: None, max_size: None, mode: Measured)
	/// Storage: Commitments CommitmentOf (r:1 w:1)
	/// Proof Skipped: Commitments CommitmentOf (max_values: None, max_size: None, mode: Measured)
	fn set_commitment() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `697`
		//  Estimated: `6344`
		// Minimum execution time: 28_000_000 picoseconds.
		Weight::from_parts(28_000_000, 6344)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
}