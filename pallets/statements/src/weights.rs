// This file is part of Anagolay Foundation.

// Copyright (C) 2019-2022 Anagolay Foundation.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Autogenerated weights for statements
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-07-28, STEPS: `50`, REPEAT: 100, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
// statements
// --extrinsic
// *
// --execution
// wasm
// --wasm-execution
// compiled
// --heap-pages
// 4096
// --output
// ./pallets/statements/src/weights.rs
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

/// Weight functions needed for statements.
pub trait WeightInfo {
  fn create_copyright() -> Weight;
  fn create_ownership() -> Weight;
  fn revoke() -> Weight;
}

/// Weights for statements using the Substrate node and recommended hardware.
pub struct AnagolayWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for AnagolayWeight<T> {
  // Storage: Statements StatementIdsByProofId (r:1 w:1)
  // Storage: Statements StatementByStatementIdAndAccountId (r:1 w:1)
  // Storage: Statements Total (r:1 w:1)
  fn create_copyright() -> Weight {
    (30_451_000 as Weight)
      .saturating_add(T::DbWeight::get().reads(3 as Weight))
      .saturating_add(T::DbWeight::get().writes(3 as Weight))
  }
  // Storage: Statements StatementIdsByProofId (r:1 w:1)
  // Storage: Statements StatementByStatementIdAndAccountId (r:1 w:1)
  // Storage: Statements Total (r:1 w:1)
  fn create_ownership() -> Weight {
    (30_300_000 as Weight)
      .saturating_add(T::DbWeight::get().reads(3 as Weight))
      .saturating_add(T::DbWeight::get().writes(3 as Weight))
  }
  // Storage: Statements StatementByStatementIdAndAccountId (r:1 w:1)
  // Storage: Statements ParentStatementIdByStatementId (r:1 w:0)
  // Storage: Statements StatementIdsByProofId (r:1 w:1)
  // Storage: Statements Total (r:1 w:1)
  fn revoke() -> Weight {
    (32_451_000 as Weight)
      .saturating_add(T::DbWeight::get().reads(4 as Weight))
      .saturating_add(T::DbWeight::get().writes(3 as Weight))
  }
}

// For backwards compatibility and tests
impl WeightInfo for () {
  // Storage: Statements StatementIdsByProofId (r:1 w:1)
  // Storage: Statements StatementByStatementIdAndAccountId (r:1 w:1)
  // Storage: Statements Total (r:1 w:1)
  fn create_copyright() -> Weight {
    (30_451_000 as Weight)
      .saturating_add(RocksDbWeight::get().reads(3 as Weight))
      .saturating_add(RocksDbWeight::get().writes(3 as Weight))
  }
  // Storage: Statements StatementIdsByProofId (r:1 w:1)
  // Storage: Statements StatementByStatementIdAndAccountId (r:1 w:1)
  // Storage: Statements Total (r:1 w:1)
  fn create_ownership() -> Weight {
    (30_300_000 as Weight)
      .saturating_add(RocksDbWeight::get().reads(3 as Weight))
      .saturating_add(RocksDbWeight::get().writes(3 as Weight))
  }
  // Storage: Statements StatementByStatementIdAndAccountId (r:1 w:1)
  // Storage: Statements ParentStatementIdByStatementId (r:1 w:0)
  // Storage: Statements StatementIdsByProofId (r:1 w:1)
  // Storage: Statements Total (r:1 w:1)
  fn revoke() -> Weight {
    (32_451_000 as Weight)
      .saturating_add(RocksDbWeight::get().reads(4 as Weight))
      .saturating_add(RocksDbWeight::get().writes(3 as Weight))
  }
}
