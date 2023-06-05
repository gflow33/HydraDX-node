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

//! Autogenerated weights for pallet_utility
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-05, STEPS: 5, REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: None, WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/hydradx
// benchmark
// pallet
// --pallet=pallet-utility
// --chain=dev
// --extrinsic=*
// --steps=5
// --repeat=20
// --output
// utility.rs
// --template
// .maintain/pallet-weight-template-no-back.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{
    traits::Get,
    weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

use pallet_utility::weights::WeightInfo;

/// Weights for pallet_utility using the hydraDX node and recommended hardware.
pub struct HydraWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for HydraWeight<T> {
	/// The range of component `c` is `[0, 1000]`.
    fn batch(c: u32, ) -> Weight {
        // Minimum execution time: 4_252 nanoseconds.
        Weight::from_ref_time(6_091_040 as u64)            // Standard Error: 5_136
            .saturating_add(Weight::from_ref_time(1_904_406 as u64).saturating_mul(c as u64))
    }
    fn as_derivative() -> Weight {
        // Minimum execution time: 3_306 nanoseconds.
        Weight::from_ref_time(3_376_000 as u64)    }
	/// The range of component `c` is `[0, 1000]`.
    fn batch_all(c: u32, ) -> Weight {
        // Minimum execution time: 4_450 nanoseconds.
        Weight::from_ref_time(3_466_500 as u64)            // Standard Error: 4_776
            .saturating_add(Weight::from_ref_time(1_943_516 as u64).saturating_mul(c as u64))
    }
    fn dispatch_as() -> Weight {
        // Minimum execution time: 5_274 nanoseconds.
        Weight::from_ref_time(5_569_000 as u64)    }
	/// The range of component `c` is `[0, 1000]`.
    fn force_batch(c: u32, ) -> Weight {
        // Minimum execution time: 4_221 nanoseconds.
        Weight::from_ref_time(10_135_840 as u64)            // Standard Error: 4_501
            .saturating_add(Weight::from_ref_time(1_897_789 as u64).saturating_mul(c as u64))
    }
}
