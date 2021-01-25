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
    decl_error, decl_event, decl_module, decl_storage, ensure,
    traits::{Currency, EnsureOrigin},
    weights::Weight,
};
use sp_runtime::{DispatchError, DispatchResult};
use sp_std::prelude::*;

mod default_weights;
#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

type BalanceOf<T> =
    <<T as Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance;
//type NegativeImbalanceOf<T> = <<T as Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::NegativeImbalance;

pub trait WeightInfo {
    fn mint() -> Weight;
}

pub trait Trait: frame_system::Trait {
    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;

    /// The currency trait.
    type Currency: Currency<Self::AccountId>;

    /// The origin which may add or remove registrars. Root can always do this.
    type MintOrigin: EnsureOrigin<Self::Origin>;

    /// Weight information for extrinsics in this pallet.
    type WeightInfo: WeightInfo;
}

decl_storage! {
    trait Store for Module<T: Trait> as Dummy {
        /// The amount of token available for minting via a bridge.
        pub ToMint get(fn to_mint) config(): BalanceOf<T>;
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
        TooMuch
    }
}

decl_module! {
    /// Identity module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        type Error = Error<T>;

        fn deposit_event() = default;

        /// Add up to `amount` of token  to `account` balance increasing total issuance.
        #[weight = T::WeightInfo::mint() ]
        fn mint(origin, account: T::AccountId, #[compact] amount: BalanceOf<T>) -> DispatchResult {
            T::MintOrigin::ensure_origin(origin)?;

            let endowed = <ToMint<T>>::try_mutate(
                | minted_amount | -> Result< BalanceOf<T>, DispatchError> {
                    ensure!( *minted_amount>=amount , Error::<T>::TooMuch);

                    *minted_amount =  *minted_amount - amount;
                    let _ = T::Currency::deposit_creating(
                        &account,
                        amount,
                    );
                    Ok( amount )
                }
            )?;

            Self::deposit_event(RawEvent::Minted(account, endowed));

            Ok(())
        }
    }
}

impl<T: Trait> Module<T> {}
