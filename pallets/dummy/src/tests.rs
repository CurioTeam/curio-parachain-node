// This file is part of Curio project.

// Copyright (C) 2021 Mixbytes.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Tests for the module.

use super::*;
use frame_support::{assert_noop, assert_ok};
use mock::{new_test_ext, Balances, Dummy, Test};

#[test]
fn test_setup_works() {
    use frame_system::RawOrigin;
    new_test_ext(1).execute_with(|| {
        assert_eq!(Balances::total_issuance(), 1_000_000);
        assert_ok!(Dummy::mint(RawOrigin::Root.into(), 2, 400));
        assert_eq!(Balances::free_balance(2), 400);
        assert_eq!(Balances::total_issuance(), 1_000_000);

        assert_noop!(
            Dummy::mint(RawOrigin::Root.into(), 2, 1_000_000),
            pallet_balances::Error::<Test, _>::InsufficientBalance
        );

        assert_eq!(Balances::free_balance(Dummy::account_id()), 1_000_000 - 400);
    });
}
