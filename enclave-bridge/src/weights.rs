/*
	Copyright 2021 Integritee AG and Supercomputing Systems AG

	Licensed under the MICROSOFT REFERENCE SOURCE LICENSE (MS-RSL) (the "License");
	you may not use this file except in compliance with the License.
	You may obtain a copy of the License at

		https://referencesource.microsoft.com/license.html

	Unless required by applicable law or agreed to in writing, software
	distributed under the License is distributed on an "AS IS" BASIS,
	WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
	See the License for the specific language governing permissions and
	limitations under the License.

*/

//! Autogenerated weights for pallet_teerex with reference hardware:
//! * Core(TM) i7-10875H
//! * 32GB of RAM
//! * NVMe SSD
//!
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-07-08, STEPS: `[50, ]`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("integritee-rococo-local-dev"), DB CACHE: 128

// Executed Command:
// ./target/release/integritee-collator
// benchmark
// --chain=integritee-rococo-local-dev
// --steps=50
// --repeat=20
// --pallet=pallet_teerex
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./polkadot-parachains/integritee-runtime/src/weights/pallet_teerex.rs
// --template=./scripts/frame-weight-template.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_teerex.
pub trait WeightInfo {
	fn invoke() -> Weight;
	fn confirm_processed_parentchain_block() -> Weight;
	fn shield_funds() -> Weight;
	fn unshield_funds() -> Weight;
	fn publish_hash(l: u32, t: u32) -> Weight;
	fn update_shard_config() -> Weight;
}

/// Weights for pallet_teerex using the Integritee parachain node and recommended hardware.
pub struct IntegriteeWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for IntegriteeWeight<T> {
	fn invoke() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 21_800 nanoseconds.
		Weight::from_parts(26_600_000, 0u64).saturating_add(Weight::from_parts(0u64, 0))
	}
	/// Storage: Teerex EnclaveIndex (r:1 w:0)
	/// Proof Skipped: Teerex EnclaveIndex (max_values: None, max_size: None, mode: Measured)
	fn confirm_processed_parentchain_block() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `242`
		//  Estimated: `2717`
		// Minimum execution time: 27_800 nanoseconds.
		Weight::from_parts(28_700_000, 0u64)
			.saturating_add(Weight::from_parts(0u64, 2717))
			.saturating_add(T::DbWeight::get().reads(1))
	}

	fn shield_funds() -> Weight {
		Weight::from_parts(26_600_000, 0u64).saturating_add(Weight::from_parts(0u64, 0))
	}

	fn unshield_funds() -> Weight {
		Weight::from_parts(26_600_000, 0u64).saturating_add(Weight::from_parts(0u64, 0))
	}

	/// Storage: Teerex EnclaveIndex (r:1 w:0)
	/// Proof Skipped: Teerex EnclaveIndex (max_values: None, max_size: None, mode: Measured)
	/// Storage: Teerex EnclaveRegistry (r:1 w:0)
	/// Proof Skipped: Teerex EnclaveRegistry (max_values: None, max_size: None, mode: Measured)
	/// Storage: System EventTopics (r:6 w:6)
	/// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
	/// The range of component `l` is `[0, 100]`.
	/// The range of component `t` is `[1, 5]`.
	fn publish_hash(l: u32, t: u32) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `362`
		//  Estimated: `8511 + t * (2475 ±0)`
		// Minimum execution time: 41_400 nanoseconds.
		Weight::from_parts(28_179_644, 0u64)
			.saturating_add(Weight::from_parts(0u64, 8511))
			// Standard Error: 21_336
			.saturating_add(Weight::from_parts(318_271, 0u64).saturating_mul(l.into()))
			// Standard Error: 467_546
			.saturating_add(Weight::from_parts(7_329_090, 0u64).saturating_mul(t.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(t.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(t.into())))
			.saturating_add(Weight::from_parts(0u64, 2475).saturating_mul(t.into()))
	}
	/// Storage: Teerex SovereignEnclaves (r:1 w:0)
	/// Proof Skipped: Teerex SovereignEnclaves (max_values: None, max_size: None, mode: Measured)
	/// Storage: EnclaveBridge ShardConfigRegistry (r:1 w:1)
	/// Proof Skipped: EnclaveBridge ShardConfigRegistry (max_values: None, max_size: None, mode: Measured)
	/// Storage: EnclaveBridge ShardStatus (r:1 w:1)
	/// Proof Skipped: EnclaveBridge ShardStatus (max_values: None, max_size: None, mode: Measured)
	fn update_shard_config() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `463`
		//  Estimated: `3928`
		// Minimum execution time: 27_734_000 picoseconds.
		Weight::from_parts(28_742_000, 0)
			.saturating_add(Weight::from_parts(0, 3928))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}

/// For tests, weights have been generated with the integritee-node.
impl WeightInfo for () {
	fn invoke() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 21_800 nanoseconds.
		Weight::from_parts(26_600_000, 0u64).saturating_add(Weight::from_parts(0u64, 0))
	}
	/// Storage: Teerex EnclaveIndex (r:1 w:0)
	/// Proof Skipped: Teerex EnclaveIndex (max_values: None, max_size: None, mode: Measured)
	fn confirm_processed_parentchain_block() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `242`
		//  Estimated: `2717`
		// Minimum execution time: 27_800 nanoseconds.
		Weight::from_parts(28_700_000, 0u64)
			.saturating_add(Weight::from_parts(0u64, 2717))
			.saturating_add(RocksDbWeight::get().reads(1))
	}

	fn shield_funds() -> Weight {
		Weight::from_parts(26_600_000, 0u64).saturating_add(Weight::from_parts(0u64, 0))
	}

	fn unshield_funds() -> Weight {
		Weight::from_parts(26_600_000, 0u64).saturating_add(Weight::from_parts(0u64, 0))
	}

	/// Storage: Teerex EnclaveIndex (r:1 w:0)
	/// Proof Skipped: Teerex EnclaveIndex (max_values: None, max_size: None, mode: Measured)
	/// Storage: Teerex EnclaveRegistry (r:1 w:0)
	/// Proof Skipped: Teerex EnclaveRegistry (max_values: None, max_size: None, mode: Measured)
	/// Storage: System EventTopics (r:6 w:6)
	/// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
	/// The range of component `l` is `[0, 100]`.
	/// The range of component `t` is `[1, 5]`.
	fn publish_hash(l: u32, t: u32) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `362`
		//  Estimated: `8511 + t * (2475 ±0)`
		// Minimum execution time: 41_400 nanoseconds.
		Weight::from_parts(28_179_644, 0u64)
			.saturating_add(Weight::from_parts(0u64, 8511))
			// Standard Error: 21_336
			.saturating_add(Weight::from_parts(318_271, 0u64).saturating_mul(l.into()))
			// Standard Error: 467_546
			.saturating_add(Weight::from_parts(7_329_090, 0u64).saturating_mul(t.into()))
			.saturating_add(RocksDbWeight::get().reads(3))
			.saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(t.into())))
			.saturating_add(RocksDbWeight::get().writes(1))
			.saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(t.into())))
			.saturating_add(Weight::from_parts(0u64, 2475).saturating_mul(t.into()))
	}

	fn update_shard_config() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `463`
		//  Estimated: `3928`
		// Minimum execution time: 27_734_000 picoseconds.
		Weight::from_parts(28_742_000, 0)
			.saturating_add(Weight::from_parts(0, 3928))
			.saturating_add(RocksDbWeight::get().reads(3))
			.saturating_add(RocksDbWeight::get().writes(2))
	}
}
