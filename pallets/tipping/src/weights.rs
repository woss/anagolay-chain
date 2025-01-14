// This file is part of Anagolay Network.

// Copyright (C) 2019-2023 Anagolay Network.

//! Autogenerated weights for tipping
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-01-16, STEPS: `50`, REPEAT: 100, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/anagolay
// benchmark
// pallet
// --chain
// dev
// --steps
// 50
// --repeat
// 100
// --pallet
// tipping
// --extrinsic
// *
// --execution
// wasm
// --wasm-execution
// compiled
// --heap-pages
// 4096
// --output
// ./pallets/tipping/src/weights.rs
// --template
// ./templates/module-weight-template.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{
  sp_std::marker::PhantomData,
  traits::Get,
  weights::{constants::RocksDbWeight, Weight},
};

/// Weight functions needed for tipping.
pub trait WeightInfo {
  fn update_settings() -> Weight;
  fn tip() -> Weight;
}

/// Weights for tipping using the Substrate node and recommended hardware.
pub struct AnagolayWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for AnagolayWeight<T> {
  // Storage: Verification AccountIdsByVerificationContext (r:1 w:0)
  fn update_settings() -> Weight {
    Weight::from_ref_time(19_480_000).saturating_add(T::DbWeight::get().reads(1))
  }
  // Storage: Verification AccountIdsByVerificationContext (r:1 w:0)
  // Storage: Verification VerificationRequestByAccountIdAndVerificationContext (r:1 w:0)
  // Storage: Tipping TippingSettingsByAccountIdAndVerificationContext (r:1 w:0)
  // Storage: Timestamp Now (r:1 w:0)
  // Storage: Tipping TipsByAccountIdAndVerificationContext (r:1 w:1)
  fn tip() -> Weight {
    Weight::from_ref_time(64_690_000)
      .saturating_add(T::DbWeight::get().reads(5))
      .saturating_add(T::DbWeight::get().writes(1))
  }
}

// For backwards compatibility and tests
impl WeightInfo for () {
  // Storage: Verification AccountIdsByVerificationContext (r:1 w:0)
  fn update_settings() -> Weight {
    Weight::from_ref_time(19_480_000).saturating_add(RocksDbWeight::get().reads(1))
  }
  // Storage: Verification AccountIdsByVerificationContext (r:1 w:0)
  // Storage: Verification VerificationRequestByAccountIdAndVerificationContext (r:1 w:0)
  // Storage: Tipping TippingSettingsByAccountIdAndVerificationContext (r:1 w:0)
  // Storage: Timestamp Now (r:1 w:0)
  // Storage: Tipping TipsByAccountIdAndVerificationContext (r:1 w:1)
  fn tip() -> Weight {
    Weight::from_ref_time(64_690_000)
      .saturating_add(RocksDbWeight::get().reads(5))
      .saturating_add(RocksDbWeight::get().writes(1))
  }
}
