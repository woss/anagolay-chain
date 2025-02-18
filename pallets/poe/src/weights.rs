// This file is part of Anagolay Network.

// Copyright (C) 2019-2023 Anagolay Network.
//! Autogenerated weights for poe
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-01-06, STEPS: `50`, REPEAT: 100, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `anagolay-anagolay-yq922khj19q`, CPU: `AMD EPYC 7B13`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/anagolay
// benchmark
// pallet
// --chain
// dev
// --steps
// 50
// --repeat
// 100
// --pallet
// poe
// --extrinsic
// *
// --execution
// wasm
// --wasm-execution
// compiled
// --heap-pages
// 4096
// --output
// ./pallets/poe/src/weights.rs
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

/// Weight functions needed for poe.
pub trait WeightInfo {
  fn create_proof() -> Weight;
  fn save_phash() -> Weight;
}

/// Weights for poe using the Substrate node and recommended hardware.
pub struct AnagolayWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for AnagolayWeight<T> {
  // Storage: Workflows WorkflowByWorkflowIdAndAccountId (r:2 w:0)
  // Storage: Poe ProofByProofIdAndAccountId (r:1 w:1)
  // Storage: Poe ProofTotal (r:1 w:1)
  fn create_proof() -> Weight {
    // Minimum execution time: 62_580 nanoseconds.
    Weight::from_ref_time(66_220_000)
      .saturating_add(T::DbWeight::get().reads(4))
      .saturating_add(T::DbWeight::get().writes(2))
  }
  // Storage: Poe ProofByProofIdAndAccountId (r:1 w:0)
  // Storage: Poe PhashByHashAndAccountId (r:1 w:1)
  // Storage: Poe PhashTotal (r:1 w:1)
  fn save_phash() -> Weight {
    // Minimum execution time: 47_800 nanoseconds.
    Weight::from_ref_time(49_330_000)
      .saturating_add(T::DbWeight::get().reads(3))
      .saturating_add(T::DbWeight::get().writes(2))
  }
}

// For backwards compatibility and tests
impl WeightInfo for () {
  // Storage: Workflows WorkflowByWorkflowIdAndAccountId (r:2 w:0)
  // Storage: Poe ProofByProofIdAndAccountId (r:1 w:1)
  // Storage: Poe ProofTotal (r:1 w:1)
  fn create_proof() -> Weight {
    // Minimum execution time: 62_580 nanoseconds.
    Weight::from_ref_time(66_220_000)
      .saturating_add(RocksDbWeight::get().reads(4))
      .saturating_add(RocksDbWeight::get().writes(2))
  }
  // Storage: Poe ProofByProofIdAndAccountId (r:1 w:0)
  // Storage: Poe PhashByHashAndAccountId (r:1 w:1)
  // Storage: Poe PhashTotal (r:1 w:1)
  fn save_phash() -> Weight {
    // Minimum execution time: 47_800 nanoseconds.
    Weight::from_ref_time(49_330_000)
      .saturating_add(RocksDbWeight::get().reads(3))
      .saturating_add(RocksDbWeight::get().writes(2))
  }
}
