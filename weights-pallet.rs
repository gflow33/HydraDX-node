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

//! Autogenerated weights for pallet_dca
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-15, STEPS: 5, REPEAT: 50, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("local"), DB CACHE: 1024

// Executed Command:
// target/release/hydradx
// benchmark
// pallet
// --pallet=pallet-dca
// --chain=local
// --steps=5
// --repeat=50
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output
// weights-pallet.rs
// --template
// .maintain/pallet-weight-template.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{
    traits::Get,
    weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_dca.
pub trait WeightInfo {
    fn on_initialize_with_buy_trade() -> Weight;
    fn on_initialize_with_sell_trade() -> Weight;
    fn on_initialize_with_empty_block() -> Weight;
    fn schedule() -> Weight;
    fn terminate() -> Weight;
}

/// Weights for pallet_dca using the hydraDX node and recommended hardware.
pub struct HydraWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for HydraWeight<T> {
	// Storage: DCA ScheduleIdsPerBlock (r:12 w:2)
	// Proof: DCA ScheduleIdsPerBlock (max_values: None, max_size: Some(101), added: 2576, mode: MaxEncodedLen)
	// Storage: DCA Schedules (r:1 w:0)
	// Proof: DCA Schedules (max_values: None, max_size: Some(191), added: 2666, mode: MaxEncodedLen)
	// Storage: DCA RemainingAmounts (r:1 w:1)
	// Proof: DCA RemainingAmounts (max_values: None, max_size: Some(36), added: 2511, mode: MaxEncodedLen)
	// Storage: Balances Reserves (r:1 w:1)
	// Proof: Balances Reserves (max_values: None, max_size: Some(1249), added: 3724, mode: MaxEncodedLen)
	// Storage: System Account (r:3 w:3)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: Omnipool Assets (r:2 w:2)
	// Proof: Omnipool Assets (max_values: None, max_size: Some(85), added: 2560, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:3 w:3)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: EmaOracle Oracles (r:4 w:0)
	// Proof: EmaOracle Oracles (max_values: None, max_size: Some(177), added: 2652, mode: MaxEncodedLen)
	// Storage: Omnipool HubAssetImbalance (r:1 w:1)
	// Proof: Omnipool HubAssetImbalance (max_values: Some(1), max_size: Some(17), added: 512, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:2 w:0)
	// Proof Skipped: AssetRegistry Assets (max_values: None, max_size: None, mode: Measured)
	// Storage: MultiTransactionPayment AccountCurrencyMap (r:1 w:0)
	// Proof: MultiTransactionPayment AccountCurrencyMap (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	// Storage: MultiTransactionPayment AcceptedCurrencies (r:1 w:0)
	// Proof: MultiTransactionPayment AcceptedCurrencies (max_values: None, max_size: Some(28), added: 2503, mode: MaxEncodedLen)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(28), added: 2503, mode: MaxEncodedLen)
	// Storage: EmaOracle Accumulator (r:1 w:1)
	// Proof: EmaOracle Accumulator (max_values: Some(1), max_size: Some(2961), added: 3456, mode: MaxEncodedLen)
	// Storage: CircuitBreaker AllowedTradeVolumeLimitPerAsset (r:2 w:2)
	// Proof: CircuitBreaker AllowedTradeVolumeLimitPerAsset (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	// Storage: DCA RetriesOnError (r:0 w:1)
	// Proof: DCA RetriesOnError (max_values: None, max_size: Some(21), added: 2496, mode: MaxEncodedLen)
    fn on_initialize_with_buy_trade() -> Weight {
        // Minimum execution time: 391_020 nanoseconds.
        Weight::from_ref_time(396_290_000 as u64)            .saturating_add(T::DbWeight::get().reads(36 as u64))
            .saturating_add(T::DbWeight::get().writes(18 as u64))
    }
	// Storage: DCA ScheduleIdsPerBlock (r:12 w:2)
	// Proof: DCA ScheduleIdsPerBlock (max_values: None, max_size: Some(101), added: 2576, mode: MaxEncodedLen)
	// Storage: DCA Schedules (r:1 w:0)
	// Proof: DCA Schedules (max_values: None, max_size: Some(191), added: 2666, mode: MaxEncodedLen)
	// Storage: DCA RemainingAmounts (r:1 w:1)
	// Proof: DCA RemainingAmounts (max_values: None, max_size: Some(36), added: 2511, mode: MaxEncodedLen)
	// Storage: Balances Reserves (r:1 w:1)
	// Proof: Balances Reserves (max_values: None, max_size: Some(1249), added: 3724, mode: MaxEncodedLen)
	// Storage: System Account (r:3 w:3)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: Omnipool Assets (r:2 w:2)
	// Proof: Omnipool Assets (max_values: None, max_size: Some(85), added: 2560, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:3 w:3)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: EmaOracle Oracles (r:4 w:0)
	// Proof: EmaOracle Oracles (max_values: None, max_size: Some(177), added: 2652, mode: MaxEncodedLen)
	// Storage: Omnipool HubAssetImbalance (r:1 w:1)
	// Proof: Omnipool HubAssetImbalance (max_values: Some(1), max_size: Some(17), added: 512, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:2 w:0)
	// Proof Skipped: AssetRegistry Assets (max_values: None, max_size: None, mode: Measured)
	// Storage: MultiTransactionPayment AccountCurrencyMap (r:1 w:0)
	// Proof: MultiTransactionPayment AccountCurrencyMap (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	// Storage: MultiTransactionPayment AcceptedCurrencies (r:1 w:0)
	// Proof: MultiTransactionPayment AcceptedCurrencies (max_values: None, max_size: Some(28), added: 2503, mode: MaxEncodedLen)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(28), added: 2503, mode: MaxEncodedLen)
	// Storage: EmaOracle Accumulator (r:1 w:1)
	// Proof: EmaOracle Accumulator (max_values: Some(1), max_size: Some(2961), added: 3456, mode: MaxEncodedLen)
	// Storage: CircuitBreaker AllowedTradeVolumeLimitPerAsset (r:2 w:2)
	// Proof: CircuitBreaker AllowedTradeVolumeLimitPerAsset (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	// Storage: DCA RetriesOnError (r:0 w:1)
	// Proof: DCA RetriesOnError (max_values: None, max_size: Some(21), added: 2496, mode: MaxEncodedLen)
    fn on_initialize_with_sell_trade() -> Weight {
        // Minimum execution time: 390_420 nanoseconds.
        Weight::from_ref_time(395_490_000 as u64)            .saturating_add(T::DbWeight::get().reads(36 as u64))
            .saturating_add(T::DbWeight::get().writes(18 as u64))
    }
	// Storage: DCA ScheduleIdsPerBlock (r:1 w:0)
	// Proof: DCA ScheduleIdsPerBlock (max_values: None, max_size: Some(101), added: 2576, mode: MaxEncodedLen)
    fn on_initialize_with_empty_block() -> Weight {
        // Minimum execution time: 4_920 nanoseconds.
        Weight::from_ref_time(5_090_000 as u64)            .saturating_add(T::DbWeight::get().reads(1 as u64))
    }
	// Storage: Omnipool Assets (r:2 w:0)
	// Proof: Omnipool Assets (max_values: None, max_size: Some(85), added: 2560, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:1 w:0)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: Omnipool HubAssetImbalance (r:1 w:0)
	// Proof: Omnipool HubAssetImbalance (max_values: Some(1), max_size: Some(17), added: 512, mode: MaxEncodedLen)
	// Storage: System Account (r:2 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: DCA ScheduleIdSequencer (r:1 w:1)
	// Proof: DCA ScheduleIdSequencer (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: Balances Reserves (r:1 w:1)
	// Proof: Balances Reserves (max_values: None, max_size: Some(1249), added: 3724, mode: MaxEncodedLen)
	// Storage: DCA ScheduleIdsPerBlock (r:11 w:1)
	// Proof: DCA ScheduleIdsPerBlock (max_values: None, max_size: Some(101), added: 2576, mode: MaxEncodedLen)
	// Storage: DCA RetriesOnError (r:0 w:1)
	// Proof: DCA RetriesOnError (max_values: None, max_size: Some(21), added: 2496, mode: MaxEncodedLen)
	// Storage: DCA Schedules (r:0 w:1)
	// Proof: DCA Schedules (max_values: None, max_size: Some(191), added: 2666, mode: MaxEncodedLen)
	// Storage: DCA ScheduleOwnership (r:0 w:1)
	// Proof: DCA ScheduleOwnership (max_values: None, max_size: Some(60), added: 2535, mode: MaxEncodedLen)
	// Storage: DCA RemainingAmounts (r:0 w:1)
	// Proof: DCA RemainingAmounts (max_values: None, max_size: Some(36), added: 2511, mode: MaxEncodedLen)
    fn schedule() -> Weight {
        // Minimum execution time: 117_740 nanoseconds.
        Weight::from_ref_time(119_990_000 as u64)            .saturating_add(T::DbWeight::get().reads(19 as u64))
            .saturating_add(T::DbWeight::get().writes(8 as u64))
    }
	// Storage: DCA Schedules (r:1 w:1)
	// Proof: DCA Schedules (max_values: None, max_size: Some(191), added: 2666, mode: MaxEncodedLen)
	// Storage: DCA RemainingAmounts (r:1 w:1)
	// Proof: DCA RemainingAmounts (max_values: None, max_size: Some(36), added: 2511, mode: MaxEncodedLen)
	// Storage: Balances Reserves (r:1 w:1)
	// Proof: Balances Reserves (max_values: None, max_size: Some(1249), added: 3724, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: DCA ScheduleIdsPerBlock (r:1 w:1)
	// Proof: DCA ScheduleIdsPerBlock (max_values: None, max_size: Some(101), added: 2576, mode: MaxEncodedLen)
	// Storage: DCA RetriesOnError (r:0 w:1)
	// Proof: DCA RetriesOnError (max_values: None, max_size: Some(21), added: 2496, mode: MaxEncodedLen)
	// Storage: DCA ScheduleOwnership (r:0 w:1)
	// Proof: DCA ScheduleOwnership (max_values: None, max_size: Some(60), added: 2535, mode: MaxEncodedLen)
    fn terminate() -> Weight {
        // Minimum execution time: 47_260 nanoseconds.
        Weight::from_ref_time(48_330_000 as u64)            .saturating_add(T::DbWeight::get().reads(5 as u64))
            .saturating_add(T::DbWeight::get().writes(7 as u64))
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: DCA ScheduleIdsPerBlock (r:12 w:2)
	// Proof: DCA ScheduleIdsPerBlock (max_values: None, max_size: Some(101), added: 2576, mode: MaxEncodedLen)
	// Storage: DCA Schedules (r:1 w:0)
	// Proof: DCA Schedules (max_values: None, max_size: Some(191), added: 2666, mode: MaxEncodedLen)
	// Storage: DCA RemainingAmounts (r:1 w:1)
	// Proof: DCA RemainingAmounts (max_values: None, max_size: Some(36), added: 2511, mode: MaxEncodedLen)
	// Storage: Balances Reserves (r:1 w:1)
	// Proof: Balances Reserves (max_values: None, max_size: Some(1249), added: 3724, mode: MaxEncodedLen)
	// Storage: System Account (r:3 w:3)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: Omnipool Assets (r:2 w:2)
	// Proof: Omnipool Assets (max_values: None, max_size: Some(85), added: 2560, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:3 w:3)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: EmaOracle Oracles (r:4 w:0)
	// Proof: EmaOracle Oracles (max_values: None, max_size: Some(177), added: 2652, mode: MaxEncodedLen)
	// Storage: Omnipool HubAssetImbalance (r:1 w:1)
	// Proof: Omnipool HubAssetImbalance (max_values: Some(1), max_size: Some(17), added: 512, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:2 w:0)
	// Proof Skipped: AssetRegistry Assets (max_values: None, max_size: None, mode: Measured)
	// Storage: MultiTransactionPayment AccountCurrencyMap (r:1 w:0)
	// Proof: MultiTransactionPayment AccountCurrencyMap (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	// Storage: MultiTransactionPayment AcceptedCurrencies (r:1 w:0)
	// Proof: MultiTransactionPayment AcceptedCurrencies (max_values: None, max_size: Some(28), added: 2503, mode: MaxEncodedLen)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(28), added: 2503, mode: MaxEncodedLen)
	// Storage: EmaOracle Accumulator (r:1 w:1)
	// Proof: EmaOracle Accumulator (max_values: Some(1), max_size: Some(2961), added: 3456, mode: MaxEncodedLen)
	// Storage: CircuitBreaker AllowedTradeVolumeLimitPerAsset (r:2 w:2)
	// Proof: CircuitBreaker AllowedTradeVolumeLimitPerAsset (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	// Storage: DCA RetriesOnError (r:0 w:1)
	// Proof: DCA RetriesOnError (max_values: None, max_size: Some(21), added: 2496, mode: MaxEncodedLen)
    fn on_initialize_with_buy_trade() -> Weight {
        // Minimum execution time: 391_020 nanoseconds.
        Weight::from_ref_time(396_290_000)
            .saturating_add(RocksDbWeight::get().reads(36))
            .saturating_add(RocksDbWeight::get().writes(18))
    }
	// Storage: DCA ScheduleIdsPerBlock (r:12 w:2)
	// Proof: DCA ScheduleIdsPerBlock (max_values: None, max_size: Some(101), added: 2576, mode: MaxEncodedLen)
	// Storage: DCA Schedules (r:1 w:0)
	// Proof: DCA Schedules (max_values: None, max_size: Some(191), added: 2666, mode: MaxEncodedLen)
	// Storage: DCA RemainingAmounts (r:1 w:1)
	// Proof: DCA RemainingAmounts (max_values: None, max_size: Some(36), added: 2511, mode: MaxEncodedLen)
	// Storage: Balances Reserves (r:1 w:1)
	// Proof: Balances Reserves (max_values: None, max_size: Some(1249), added: 3724, mode: MaxEncodedLen)
	// Storage: System Account (r:3 w:3)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: Omnipool Assets (r:2 w:2)
	// Proof: Omnipool Assets (max_values: None, max_size: Some(85), added: 2560, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:3 w:3)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: EmaOracle Oracles (r:4 w:0)
	// Proof: EmaOracle Oracles (max_values: None, max_size: Some(177), added: 2652, mode: MaxEncodedLen)
	// Storage: Omnipool HubAssetImbalance (r:1 w:1)
	// Proof: Omnipool HubAssetImbalance (max_values: Some(1), max_size: Some(17), added: 512, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:2 w:0)
	// Proof Skipped: AssetRegistry Assets (max_values: None, max_size: None, mode: Measured)
	// Storage: MultiTransactionPayment AccountCurrencyMap (r:1 w:0)
	// Proof: MultiTransactionPayment AccountCurrencyMap (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	// Storage: MultiTransactionPayment AcceptedCurrencies (r:1 w:0)
	// Proof: MultiTransactionPayment AcceptedCurrencies (max_values: None, max_size: Some(28), added: 2503, mode: MaxEncodedLen)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(28), added: 2503, mode: MaxEncodedLen)
	// Storage: EmaOracle Accumulator (r:1 w:1)
	// Proof: EmaOracle Accumulator (max_values: Some(1), max_size: Some(2961), added: 3456, mode: MaxEncodedLen)
	// Storage: CircuitBreaker AllowedTradeVolumeLimitPerAsset (r:2 w:2)
	// Proof: CircuitBreaker AllowedTradeVolumeLimitPerAsset (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	// Storage: DCA RetriesOnError (r:0 w:1)
	// Proof: DCA RetriesOnError (max_values: None, max_size: Some(21), added: 2496, mode: MaxEncodedLen)
    fn on_initialize_with_sell_trade() -> Weight {
        // Minimum execution time: 390_420 nanoseconds.
        Weight::from_ref_time(395_490_000)
            .saturating_add(RocksDbWeight::get().reads(36))
            .saturating_add(RocksDbWeight::get().writes(18))
    }
	// Storage: DCA ScheduleIdsPerBlock (r:1 w:0)
	// Proof: DCA ScheduleIdsPerBlock (max_values: None, max_size: Some(101), added: 2576, mode: MaxEncodedLen)
    fn on_initialize_with_empty_block() -> Weight {
        // Minimum execution time: 4_920 nanoseconds.
        Weight::from_ref_time(5_090_000)
            .saturating_add(RocksDbWeight::get().reads(1))
    }
	// Storage: Omnipool Assets (r:2 w:0)
	// Proof: Omnipool Assets (max_values: None, max_size: Some(85), added: 2560, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:1 w:0)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: Omnipool HubAssetImbalance (r:1 w:0)
	// Proof: Omnipool HubAssetImbalance (max_values: Some(1), max_size: Some(17), added: 512, mode: MaxEncodedLen)
	// Storage: System Account (r:2 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: DCA ScheduleIdSequencer (r:1 w:1)
	// Proof: DCA ScheduleIdSequencer (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: Balances Reserves (r:1 w:1)
	// Proof: Balances Reserves (max_values: None, max_size: Some(1249), added: 3724, mode: MaxEncodedLen)
	// Storage: DCA ScheduleIdsPerBlock (r:11 w:1)
	// Proof: DCA ScheduleIdsPerBlock (max_values: None, max_size: Some(101), added: 2576, mode: MaxEncodedLen)
	// Storage: DCA RetriesOnError (r:0 w:1)
	// Proof: DCA RetriesOnError (max_values: None, max_size: Some(21), added: 2496, mode: MaxEncodedLen)
	// Storage: DCA Schedules (r:0 w:1)
	// Proof: DCA Schedules (max_values: None, max_size: Some(191), added: 2666, mode: MaxEncodedLen)
	// Storage: DCA ScheduleOwnership (r:0 w:1)
	// Proof: DCA ScheduleOwnership (max_values: None, max_size: Some(60), added: 2535, mode: MaxEncodedLen)
	// Storage: DCA RemainingAmounts (r:0 w:1)
	// Proof: DCA RemainingAmounts (max_values: None, max_size: Some(36), added: 2511, mode: MaxEncodedLen)
    fn schedule() -> Weight {
        // Minimum execution time: 117_740 nanoseconds.
        Weight::from_ref_time(119_990_000)
            .saturating_add(RocksDbWeight::get().reads(19))
            .saturating_add(RocksDbWeight::get().writes(8))
    }
	// Storage: DCA Schedules (r:1 w:1)
	// Proof: DCA Schedules (max_values: None, max_size: Some(191), added: 2666, mode: MaxEncodedLen)
	// Storage: DCA RemainingAmounts (r:1 w:1)
	// Proof: DCA RemainingAmounts (max_values: None, max_size: Some(36), added: 2511, mode: MaxEncodedLen)
	// Storage: Balances Reserves (r:1 w:1)
	// Proof: Balances Reserves (max_values: None, max_size: Some(1249), added: 3724, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: DCA ScheduleIdsPerBlock (r:1 w:1)
	// Proof: DCA ScheduleIdsPerBlock (max_values: None, max_size: Some(101), added: 2576, mode: MaxEncodedLen)
	// Storage: DCA RetriesOnError (r:0 w:1)
	// Proof: DCA RetriesOnError (max_values: None, max_size: Some(21), added: 2496, mode: MaxEncodedLen)
	// Storage: DCA ScheduleOwnership (r:0 w:1)
	// Proof: DCA ScheduleOwnership (max_values: None, max_size: Some(60), added: 2535, mode: MaxEncodedLen)
    fn terminate() -> Weight {
        // Minimum execution time: 47_260 nanoseconds.
        Weight::from_ref_time(48_330_000)
            .saturating_add(RocksDbWeight::get().reads(5))
            .saturating_add(RocksDbWeight::get().writes(7))
    }
}
