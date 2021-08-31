// This file is part of Anagolay Foundation.
// Copyright (C) 2019-2021 Anagolay Foundation.
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

use super::*;

impl<T: Config> Pallet<T> {
  /// Increase the Rule count
  ///
  /// Does no checks!
  ///
  /// Returns the new Total Rule count
  fn increase_rule_count() -> u32 {
    let rule_count = Self::rule_count();
    let new_rule_count = &rule_count + 1;
    <RuleCount<T>>::put(new_rule_count);

    new_rule_count
  }

  /// Save the Rule to the Storage
  ///   
  /// Increases the `Total Rule Count` via `Self::increase_rule_count`
  ///
  /// Does no checks.
  pub fn create(
    account_id: &T::AccountId,
    block_number: &T::BlockNumber,
    rule: &Rule,
  ) -> RuleInfo<T::AccountId, T::BlockNumber> {
    let rule_info = RuleInfo {
      rule: rule.clone(),
      account_id: account_id.clone(),
      block_number: *block_number,
    };

    Rules::<T>::insert(&rule.id, &account_id, rule_info.clone());
    Self::increase_rule_count();

    rule_info
  }
}