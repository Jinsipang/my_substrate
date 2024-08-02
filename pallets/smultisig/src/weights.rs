
//! Autogenerated weights for `pallet_smultisig`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-08-02, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `SccdeMacBook-Pro.local`, CPU: `<UNKNOWN>`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: 1024

// Executed Command:
// ./target/release/schain
// benchmark
// pallet
// --chain
// dev
// --pallet
// pallet_smultisig
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// pallets/smultisig/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_smultisig`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_smultisig::WeightInfo for WeightInfo<T> {
	/// Storage: `Smultisig::MultisigMembers` (r:1 w:1)
	/// Proof: `Smultisig::MultisigMembers` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn create_multisig_group() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4`
		//  Estimated: `1489`
		// Minimum execution time: 4_000_000 picoseconds.
		Weight::from_parts(5_000_000, 0)
			.saturating_add(Weight::from_parts(0, 1489))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Smultisig::MultisigMembers` (r:1 w:1)
	/// Proof: `Smultisig::MultisigMembers` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn add_member() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4`
		//  Estimated: `1489`
		// Minimum execution time: 4_000_000 picoseconds.
		Weight::from_parts(5_000_000, 0)
			.saturating_add(Weight::from_parts(0, 1489))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Smultisig::MultisigMembers` (r:1 w:1)
	/// Proof: `Smultisig::MultisigMembers` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn remove_member() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4`
		//  Estimated: `1489`
		// Minimum execution time: 4_000_000 picoseconds.
		Weight::from_parts(5_000_000, 0)
			.saturating_add(Weight::from_parts(0, 1489))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Smultisig::MultisigMembers` (r:1 w:1)
	/// Proof: `Smultisig::MultisigMembers` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn approve_proposal() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4`
		//  Estimated: `1489`
		// Minimum execution time: 4_000_000 picoseconds.
		Weight::from_parts(5_000_000, 0)
			.saturating_add(Weight::from_parts(0, 1489))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Smultisig::MultisigMembers` (r:1 w:1)
	/// Proof: `Smultisig::MultisigMembers` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn reject_proposal() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4`
		//  Estimated: `1489`
		// Minimum execution time: 4_000_000 picoseconds.
		Weight::from_parts(5_000_000, 0)
			.saturating_add(Weight::from_parts(0, 1489))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
