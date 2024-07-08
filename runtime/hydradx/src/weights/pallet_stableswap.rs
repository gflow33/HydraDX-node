// This file is part of HydraDX.

// Copyright (C) 2020-2023  Intergalactic, Limited (GIB).
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.


//! Autogenerated weights for `pallet_stableswap`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-06-20, STEPS: `10`, REPEAT: `30`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bench-bot`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// target/release/hydradx
// benchmark
// pallet
// --chain=dev
// --steps=10
// --repeat=30
// --wasm-execution=compiled
// --heap-pages=4096
// --template=scripts/pallet-weight-template.hbs
// --pallet=pallet-stableswap
// --output=weights/stableswap.rs
// --extrinsic=*

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weights for `pallet_stableswap`.
pub struct WeightInfo<T>(PhantomData<T>);

/// Weights for `pallet_stableswap` using the HydraDX node and recommended hardware.
pub struct HydraWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_stableswap::WeightInfo for HydraWeight<T> {
	/// Storage: `Stableswap::Pools` (r:1 w:1)
	/// Proof: `Stableswap::Pools` (`max_values`: None, `max_size`: Some(57), added: 2532, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:6 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:0 w:1)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	fn create_pool() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `855`
		//  Estimated: `16590`
		// Minimum execution time: 42_163_000 picoseconds.
		Weight::from_parts(42_971_000, 16590)
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Stableswap::Pools` (r:1 w:0)
	/// Proof: `Stableswap::Pools` (`max_values`: None, `max_size`: Some(57), added: 2532, mode: `MaxEncodedLen`)
	/// Storage: `Stableswap::AssetTradability` (r:5 w:0)
	/// Proof: `Stableswap::AssetTradability` (`max_values`: None, `max_size`: Some(41), added: 2516, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:11 w:11)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:6 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::TotalIssuance` (r:1 w:1)
	/// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:6 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AccountCurrencyMap` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AccountCurrencyMap` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AcceptedCurrencies` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AcceptedCurrencies` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:1 w:0)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::Accumulator` (r:1 w:1)
	/// Proof: `EmaOracle::Accumulator` (`max_values`: Some(1), `max_size`: Some(5921), added: 6416, mode: `MaxEncodedLen`)
	fn add_liquidity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3471`
		//  Estimated: `29403`
		// Minimum execution time: 1_327_528_000 picoseconds.
		Weight::from_parts(1_331_117_000, 29403)
			.saturating_add(T::DbWeight::get().reads(35_u64))
			.saturating_add(T::DbWeight::get().writes(14_u64))
	}
	/// Storage: `Stableswap::AssetTradability` (r:1 w:0)
	/// Proof: `Stableswap::AssetTradability` (`max_values`: None, `max_size`: Some(41), added: 2516, mode: `MaxEncodedLen`)
	/// Storage: `Stableswap::Pools` (r:1 w:0)
	/// Proof: `Stableswap::Pools` (`max_values`: None, `max_size`: Some(57), added: 2532, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::TotalIssuance` (r:1 w:1)
	/// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:7 w:3)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:6 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:2 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AccountCurrencyMap` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AccountCurrencyMap` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AcceptedCurrencies` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AcceptedCurrencies` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:1 w:0)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::Accumulator` (r:1 w:1)
	/// Proof: `EmaOracle::Accumulator` (`max_values`: Some(1), `max_size`: Some(5921), added: 6416, mode: `MaxEncodedLen`)
	fn add_liquidity_shares() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3339`
		//  Estimated: `19071`
		// Minimum execution time: 896_585_000 picoseconds.
		Weight::from_parts(900_938_000, 19071)
			.saturating_add(T::DbWeight::get().reads(23_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	/// Storage: `Stableswap::AssetTradability` (r:1 w:0)
	/// Proof: `Stableswap::AssetTradability` (`max_values`: None, `max_size`: Some(41), added: 2516, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:7 w:3)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `Stableswap::Pools` (r:1 w:0)
	/// Proof: `Stableswap::Pools` (`max_values`: None, `max_size`: Some(57), added: 2532, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:6 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::TotalIssuance` (r:1 w:1)
	/// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:1 w:0)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:1 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AcceptedCurrencies` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AcceptedCurrencies` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::Accumulator` (r:1 w:1)
	/// Proof: `EmaOracle::Accumulator` (`max_values`: Some(1), `max_size`: Some(5921), added: 6416, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AccountCurrencyMap` (r:0 w:1)
	/// Proof: `MultiTransactionPayment::AccountCurrencyMap` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	fn remove_liquidity_one_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3363`
		//  Estimated: `19071`
		// Minimum execution time: 925_710_000 picoseconds.
		Weight::from_parts(928_674_000, 19071)
			.saturating_add(T::DbWeight::get().reads(22_u64))
			.saturating_add(T::DbWeight::get().writes(7_u64))
	}
	/// Storage: `Stableswap::AssetTradability` (r:1 w:0)
	/// Proof: `Stableswap::AssetTradability` (`max_values`: None, `max_size`: Some(41), added: 2516, mode: `MaxEncodedLen`)
	/// Storage: `Stableswap::Pools` (r:1 w:0)
	/// Proof: `Stableswap::Pools` (`max_values`: None, `max_size`: Some(57), added: 2532, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:7 w:3)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:6 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::TotalIssuance` (r:1 w:1)
	/// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:1 w:0)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:1 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AccountCurrencyMap` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AccountCurrencyMap` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AcceptedCurrencies` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AcceptedCurrencies` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::Accumulator` (r:1 w:1)
	/// Proof: `EmaOracle::Accumulator` (`max_values`: Some(1), `max_size`: Some(5921), added: 6416, mode: `MaxEncodedLen`)
	fn withdraw_asset_amount() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3363`
		//  Estimated: `19071`
		// Minimum execution time: 1_412_294_000 picoseconds.
		Weight::from_parts(1_416_948_000, 19071)
			.saturating_add(T::DbWeight::get().reads(23_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	/// Storage: `Stableswap::AssetTradability` (r:2 w:0)
	/// Proof: `Stableswap::AssetTradability` (`max_values`: None, `max_size`: Some(41), added: 2516, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:7 w:4)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `Stableswap::Pools` (r:1 w:0)
	/// Proof: `Stableswap::Pools` (`max_values`: None, `max_size`: Some(57), added: 2532, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:6 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:2 w:0)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:2 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AcceptedCurrencies` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AcceptedCurrencies` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::TotalIssuance` (r:1 w:0)
	/// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::Accumulator` (r:1 w:1)
	/// Proof: `EmaOracle::Accumulator` (`max_values`: Some(1), `max_size`: Some(5921), added: 6416, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AccountCurrencyMap` (r:0 w:1)
	/// Proof: `MultiTransactionPayment::AccountCurrencyMap` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	fn sell() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3426`
		//  Estimated: `19071`
		// Minimum execution time: 862_103_000 picoseconds.
		Weight::from_parts(867_533_000, 19071)
			.saturating_add(T::DbWeight::get().reads(25_u64))
			.saturating_add(T::DbWeight::get().writes(7_u64))
	}
	/// Storage: `Stableswap::AssetTradability` (r:2 w:0)
	/// Proof: `Stableswap::AssetTradability` (`max_values`: None, `max_size`: Some(41), added: 2516, mode: `MaxEncodedLen`)
	/// Storage: `Stableswap::Pools` (r:1 w:0)
	/// Proof: `Stableswap::Pools` (`max_values`: None, `max_size`: Some(57), added: 2532, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:7 w:4)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:6 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:2 w:0)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:2 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AccountCurrencyMap` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AccountCurrencyMap` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AcceptedCurrencies` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AcceptedCurrencies` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::TotalIssuance` (r:1 w:0)
	/// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::Accumulator` (r:1 w:1)
	/// Proof: `EmaOracle::Accumulator` (`max_values`: Some(1), `max_size`: Some(5921), added: 6416, mode: `MaxEncodedLen`)
	fn buy() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3396`
		//  Estimated: `19071`
		// Minimum execution time: 844_279_000 picoseconds.
		Weight::from_parts(848_664_000, 19071)
			.saturating_add(T::DbWeight::get().reads(26_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	/// Storage: `Stableswap::Pools` (r:1 w:0)
	/// Proof: `Stableswap::Pools` (`max_values`: None, `max_size`: Some(57), added: 2532, mode: `MaxEncodedLen`)
	/// Storage: `Stableswap::AssetTradability` (r:1 w:1)
	/// Proof: `Stableswap::AssetTradability` (`max_values`: None, `max_size`: Some(41), added: 2516, mode: `MaxEncodedLen`)
	fn set_asset_tradable_state() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `291`
		//  Estimated: `3522`
		// Minimum execution time: 21_172_000 picoseconds.
		Weight::from_parts(21_538_000, 3522)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Stableswap::Pools` (r:1 w:1)
	/// Proof: `Stableswap::Pools` (`max_values`: None, `max_size`: Some(57), added: 2532, mode: `MaxEncodedLen`)
	fn update_pool_fee() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `291`
		//  Estimated: `3522`
		// Minimum execution time: 18_710_000 picoseconds.
		Weight::from_parts(19_155_000, 3522)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Stableswap::Pools` (r:1 w:1)
	/// Proof: `Stableswap::Pools` (`max_values`: None, `max_size`: Some(57), added: 2532, mode: `MaxEncodedLen`)
	fn update_amplification() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `291`
		//  Estimated: `3522`
		// Minimum execution time: 19_403_000 picoseconds.
		Weight::from_parts(19_822_000, 3522)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Stableswap::Pools` (r:1 w:0)
	/// Proof: `Stableswap::Pools` (`max_values`: None, `max_size`: Some(57), added: 2532, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:7 w:4)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:6 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `Stableswap::AssetTradability` (r:2 w:0)
	/// Proof: `Stableswap::AssetTradability` (`max_values`: None, `max_size`: Some(41), added: 2516, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:2 w:0)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:2 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AcceptedCurrencies` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AcceptedCurrencies` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::TotalIssuance` (r:1 w:0)
	/// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::Accumulator` (r:1 w:1)
	/// Proof: `EmaOracle::Accumulator` (`max_values`: Some(1), `max_size`: Some(5921), added: 6416, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AccountCurrencyMap` (r:0 w:1)
	/// Proof: `MultiTransactionPayment::AccountCurrencyMap` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[1, 2]`.
	/// The range of component `e` is `[0, 1]`.
	fn router_execution_sell(_c: u32, e: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1621 + e * (1805 ±0)`
		//  Estimated: `13990 + e * (5797 ±201_913_854_157_949_888)`
		// Minimum execution time: 371_777_000 picoseconds.
		Weight::from_parts(384_499_218, 13990)
			// Standard Error: 722_175
			.saturating_add(Weight::from_parts(831_895_585, 0).saturating_mul(e.into()))
			.saturating_add(T::DbWeight::get().reads(11_u64))
			.saturating_add(T::DbWeight::get().reads((14_u64).saturating_mul(e.into())))
			.saturating_add(T::DbWeight::get().writes((7_u64).saturating_mul(e.into())))
			.saturating_add(Weight::from_parts(0, 5797).saturating_mul(e.into()))
	}
	/// Storage: `Stableswap::Pools` (r:1 w:0)
	/// Proof: `Stableswap::Pools` (`max_values`: None, `max_size`: Some(57), added: 2532, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:7 w:4)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:6 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `Stableswap::AssetTradability` (r:2 w:0)
	/// Proof: `Stableswap::AssetTradability` (`max_values`: None, `max_size`: Some(41), added: 2516, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:2 w:0)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:2 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AccountCurrencyMap` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AccountCurrencyMap` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AcceptedCurrencies` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AcceptedCurrencies` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::TotalIssuance` (r:1 w:0)
	/// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::Accumulator` (r:1 w:1)
	/// Proof: `EmaOracle::Accumulator` (`max_values`: Some(1), `max_size`: Some(5921), added: 6416, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[1, 2]`.
	/// The range of component `e` is `[0, 1]`.
	fn router_execution_buy(c: u32, e: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1620 + e * (1776 ±0)`
		//  Estimated: `13990 + e * (5797 ±213_489_293_787_578_112)`
		// Minimum execution time: 371_923_000 picoseconds.
		Weight::from_parts(373_003_000, 13990)
			// Standard Error: 3_999_909
			.saturating_add(Weight::from_parts(15_028_502, 0).saturating_mul(c.into()))
			// Standard Error: 8_780_942
			.saturating_add(Weight::from_parts(520_802_190, 0).saturating_mul(e.into()))
			.saturating_add(T::DbWeight::get().reads(11_u64))
			.saturating_add(T::DbWeight::get().reads((15_u64).saturating_mul(e.into())))
			.saturating_add(T::DbWeight::get().writes((6_u64).saturating_mul(e.into())))
			.saturating_add(Weight::from_parts(0, 5797).saturating_mul(e.into()))
	}
	/// Storage: `Stableswap::Pools` (r:1 w:0)
	/// Proof: `Stableswap::Pools` (`max_values`: None, `max_size`: Some(57), added: 2532, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:5 w:0)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:5 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::TotalIssuance` (r:1 w:0)
	/// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	fn calculate_spot_price_with_fee() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1827`
		//  Estimated: `13990`
		// Minimum execution time: 308_105_000 picoseconds.
		Weight::from_parts(309_400_000, 13990)
			.saturating_add(T::DbWeight::get().reads(12_u64))
	}
}
