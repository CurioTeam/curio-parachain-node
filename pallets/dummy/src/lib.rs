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

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    decl_error, decl_event, decl_module, decl_storage,
    traits::{Currency, EnsureOrigin, ExistenceRequirement::KeepAlive, Get},
    weights::Weight,
};
use sp_runtime::{traits::AccountIdConversion, DispatchResult, ModuleId};
use sp_std::prelude::*;

mod default_weights;
#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

type BalanceOf<T> =
    <<T as Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance;

pub trait WeightInfo {
    fn mint() -> Weight;
}

pub trait Trait: frame_system::Trait {
    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;

    /// The bridge's module id, used for deriving its sovereign account ID.
    type ModuleId: Get<ModuleId>;

    /// The currency trait.
    type Currency: Currency<Self::AccountId>;

    /// The origin which may add or remove registrars. Root can always do this.
    type MintOrigin: EnsureOrigin<Self::Origin>;

    /// Weight information for extrinsics in this pallet.
    type WeightInfo: WeightInfo;
}

decl_storage! {
    trait Store for Module<T: Trait> as Dummy {

    }
    add_extra_genesis {
        config(balance): BalanceOf<T>;
        build(|config| {
            // Create Bridge pot
            assert!(
                config.balance >= T::Currency::minimum_balance(),
                "the balance of any account should always be more than existential deposit.",
            );
            T::Currency::deposit_creating(&<Module<T>>::account_id(), config.balance);
        });
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Trait>::AccountId,
        Balance = BalanceOf<T>,
    {
        /// A mint was called. \[who, amount\]
        Minted(AccountId, Balance),
    }
);

decl_error! {
    /// Error for the identity module.
    pub enum Error for Module<T: Trait> {

    }
}

decl_module! {
    /// Identity module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        type Error = Error<T>;
        /// The bridge's module id, used for deriving its sovereign account ID.
        const ModuleId: ModuleId = T::ModuleId::get();

        fn deposit_event() = default;

        /// Add up to `amount` of token  to `account` balance increasing total issuance.
        #[weight = T::WeightInfo::mint() ]
        fn mint(origin, account: T::AccountId, #[compact] amount: BalanceOf<T>) -> DispatchResult {
            T::MintOrigin::ensure_origin(origin)?;

            let bridge = Self::account_id();
            let _ = T::Currency::transfer(&bridge, &account, amount, KeepAlive)?;

            Self::deposit_event(RawEvent::Minted(account, amount));
            Ok(())
        }
    }
}

impl<T: Trait> Module<T> {
    /// The account ID of the external chain.
    ///
    /// This actually does computation. If you need to keep using it, then make sure you cache the
    /// value and only call this once.
    pub fn account_id() -> T::AccountId {
        T::ModuleId::get().into_account()
    }
}
