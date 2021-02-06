// This file is part of Substrate.

// Copyright (C) 2018-2020 Parity Technologies (UK) Ltd.
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

//! Substrate chain configurations.

use grandpa_primitives::AuthorityId as GrandpaId;
use hex_literal::hex;
use node_runtime::constants::currency::*;
use node_runtime::Block;
use node_runtime::{
    wasm_binary_unwrap, AuthorityDiscoveryConfig, BabeConfig, BalancesConfig, ContractsConfig,
    CouncilConfig, DemocracyConfig, DummyConfig, ElectionsConfig, GrandpaConfig, ImOnlineConfig,
    IndicesConfig, SessionConfig, SessionKeys, SocietyConfig, StakerStatus, StakingConfig,
    SudoConfig, SystemConfig, TechnicalCommitteeConfig,
};
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use sc_chain_spec::ChainSpecExtension;
use sc_service::ChainType;
use sc_telemetry::TelemetryEndpoints;
use serde::{Deserialize, Serialize};
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public};
use sp_runtime::{
    traits::{IdentifyAccount, Verify},
    Perbill,
};

pub use node_primitives::{AccountId, Balance, Signature};
pub use node_runtime::GenesisConfig;

type AccountPublic = <Signature as Verify>::Signer;

const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Node `ChainSpec` extensions.
///
/// Additional parameters for some Substrate core modules,
/// customizable from the chain spec.
#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
    /// Block numbers with known hashes.
    pub fork_blocks: sc_client_api::ForkBlocks<Block>,
    /// Known bad block hashes.
    pub bad_blocks: sc_client_api::BadBlocks<Block>,
}

/// Specialized `ChainSpec`.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig, Extensions>;

fn session_keys(
    grandpa: GrandpaId,
    babe: BabeId,
    im_online: ImOnlineId,
    authority_discovery: AuthorityDiscoveryId,
) -> SessionKeys {
    SessionKeys {
        grandpa,
        babe,
        im_online,
        authority_discovery,
    }
}

fn staging_testnet_config_genesis() -> GenesisConfig {
    // use scripts/init.sh to generate keys
    let initial_authorities: Vec<(
        AccountId,
        AccountId,
        GrandpaId,
        BabeId,
        ImOnlineId,
        AuthorityDiscoveryId,
    )> = vec![
        (
            // 5E6txmNZmmsMWJX3cEAUioez5FgLge5honKxMGXPpacHMDL7
            hex!["5a2457447e5481413eb173475f56f1654f13aaed5f89a88b7e474cb86a884945"].into(),
            // 5FW1BzmTf9LoRojcMgpe759BbEbMDijsf4mXoqf5CWoDCdzh
            hex!["98002f325a021254275bd14205dbbe27e37a64f5b01b2392290fda031f662f5b"].into(),
            // 5E2M97fJnMi4Mxd9BYbMfi5Nqk4eVYyhXds1ekAtBwTpwLGz
            hex!["56ac3d05b13dbf1e2c7f48293ce538704c133a80e6b14afcd4125bbea9d70cbc"]
                .unchecked_into(),
            // 5FW1BzmTf9LoRojcMgpe759BbEbMDijsf4mXoqf5CWoDCdzh
            hex!["98002f325a021254275bd14205dbbe27e37a64f5b01b2392290fda031f662f5b"]
                .unchecked_into(),
            // 5FW1BzmTf9LoRojcMgpe759BbEbMDijsf4mXoqf5CWoDCdzh
            hex!["98002f325a021254275bd14205dbbe27e37a64f5b01b2392290fda031f662f5b"]
                .unchecked_into(),
            // 5FW1BzmTf9LoRojcMgpe759BbEbMDijsf4mXoqf5CWoDCdzh
            hex!["98002f325a021254275bd14205dbbe27e37a64f5b01b2392290fda031f662f5b"]
                .unchecked_into(),
        ),
        (
            // 5FtDS7m9xHv7NisxMCpvMYpLHznSVXJeS6nyqnAjFFGfGTqi
            hex!["a8f0cc64d5d6e2e1b7884370acf88d94350e49f2431ba4964abe05548251ea32"].into(),
            // 5F1vo3oyRKPoE1k7wdnu1Nvifzy4hit9MmVibF2GYvYgkc8m
            hex!["82968416cd6bb91f58fad8583f4fd40c2ea579630ba3ca4d5ca3724a3e66394a"].into(),
            // 5EuGSqxh27J88VKML9jEg4pyKhPemaRr6nFGhAZaDvHpN9hd
            hex!["7d81f3950b2b08c2c64eac1b5af2ca7d1f73a459188bd0a53a114993806927b4"]
                .unchecked_into(),
            // 5F1vo3oyRKPoE1k7wdnu1Nvifzy4hit9MmVibF2GYvYgkc8m
            hex!["82968416cd6bb91f58fad8583f4fd40c2ea579630ba3ca4d5ca3724a3e66394a"]
                .unchecked_into(),
            // 5F1vo3oyRKPoE1k7wdnu1Nvifzy4hit9MmVibF2GYvYgkc8m
            hex!["82968416cd6bb91f58fad8583f4fd40c2ea579630ba3ca4d5ca3724a3e66394a"]
                .unchecked_into(),
            // 5F1vo3oyRKPoE1k7wdnu1Nvifzy4hit9MmVibF2GYvYgkc8m
            hex!["82968416cd6bb91f58fad8583f4fd40c2ea579630ba3ca4d5ca3724a3e66394a"]
                .unchecked_into(),
        ),
        (
            // 5HgEeHE8CFxD3jx3JpFxTNuLxRPRu8LWcnJrvgtuXus475ua
            hex!["f8466488f2b50dc7e993b22a378d28abcb0596e633090a564972b1579f73e840"].into(),
            // 5FHebL2wLDd6ZeGEkJ2EhzxUCmu43EUPzZgkJEsvJMBPtvCV
            hex!["8e93e6c22ac2a07a95580b2aaca5d8de35c5f9da37d94f2e09ab89274727bf45"].into(),
            // 5CHyKchqmmZrnU7aTcqDGJsQ4kcYRVwRSjhFXkYxZh3mnQj2
            hex!["0a1e3fb0ba4800965f404c68bba42efe689855a541de02ee785066e906f7633a"]
                .unchecked_into(),
            // 5FHebL2wLDd6ZeGEkJ2EhzxUCmu43EUPzZgkJEsvJMBPtvCV
            hex!["8e93e6c22ac2a07a95580b2aaca5d8de35c5f9da37d94f2e09ab89274727bf45"]
                .unchecked_into(),
            // 5FHebL2wLDd6ZeGEkJ2EhzxUCmu43EUPzZgkJEsvJMBPtvCV
            hex!["8e93e6c22ac2a07a95580b2aaca5d8de35c5f9da37d94f2e09ab89274727bf45"]
                .unchecked_into(),
            // 5FHebL2wLDd6ZeGEkJ2EhzxUCmu43EUPzZgkJEsvJMBPtvCV
            hex!["8e93e6c22ac2a07a95580b2aaca5d8de35c5f9da37d94f2e09ab89274727bf45"]
                .unchecked_into(),
        ),
        (
            // 5GuSXrrWnB9wxj2YDKCfeibZ5mhCZ2rdfsU7qKrJGVvxfjva
            hex!["d61c5f5ba4348435b3e729dbb865014d103dcbb7809149dcaebb008c70314d4d"].into(),
            // 5He61m6ATsTgZQotDJUbAZFRtQg3tBQsVd6e4c1qsmydS435
            hex!["f6a2da176b324fa3d8f8726d65ad712964d4cf5c8c50649e292500e7cf683012"].into(),
            // 5HMZw42GrJDRqZ7GZWXQ1zpVBa5w3EJ2dKEkxba5ZTXagKPm
            hex!["ea09a9c7156debf058d28b02740e7056c081a69bec7b9b65878ddf490a1798a6"]
                .unchecked_into(),
            // 5He61m6ATsTgZQotDJUbAZFRtQg3tBQsVd6e4c1qsmydS435
            hex!["f6a2da176b324fa3d8f8726d65ad712964d4cf5c8c50649e292500e7cf683012"]
                .unchecked_into(),
            // 5He61m6ATsTgZQotDJUbAZFRtQg3tBQsVd6e4c1qsmydS435
            hex!["f6a2da176b324fa3d8f8726d65ad712964d4cf5c8c50649e292500e7cf683012"]
                .unchecked_into(),
            // 5He61m6ATsTgZQotDJUbAZFRtQg3tBQsVd6e4c1qsmydS435
            hex!["f6a2da176b324fa3d8f8726d65ad712964d4cf5c8c50649e292500e7cf683012"]
                .unchecked_into(),
        ),
    ];

    // generated with secret
    let root_key: AccountId = hex![
        // 5F9rKX5AzK2NQNGyEnz2HphY4Sk9wwmtDyZ7uND4xwxWxqaA
        "88a16a8aa1aa0f639df1583c9123001bb44e4c69f89df5d85a87db4423cea85e"
    ]
    .into();

    let endowed_accounts: Vec<AccountId> = vec![root_key.clone()];

    testnet_genesis(initial_authorities, root_key, Some(endowed_accounts), false)
}

/// Staging testnet config.
pub fn staging_testnet_config() -> ChainSpec {
    let boot_nodes = vec![];
    ChainSpec::from_genesis(
        "Staging Testnet",
        "staging_testnet",
        ChainType::Live,
        staging_testnet_config_genesis,
        boot_nodes,
        Some(
            TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 0)])
                .expect("Staging telemetry url is valid; qed"),
        ),
        None,
        None,
        Default::default(),
    )
}

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate stash, controller and session key from seed
pub fn authority_keys_from_seed(
    seed: &str,
) -> (
    AccountId,
    AccountId,
    GrandpaId,
    BabeId,
    ImOnlineId,
    AuthorityDiscoveryId,
) {
    (
        get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
        get_account_id_from_seed::<sr25519::Public>(seed),
        get_from_seed::<GrandpaId>(seed),
        get_from_seed::<BabeId>(seed),
        get_from_seed::<ImOnlineId>(seed),
        get_from_seed::<AuthorityDiscoveryId>(seed),
    )
}

/// Helper function to create GenesisConfig for testing
pub fn testnet_genesis(
    initial_authorities: Vec<(
        AccountId,
        AccountId,
        GrandpaId,
        BabeId,
        ImOnlineId,
        AuthorityDiscoveryId,
    )>,
    root_key: AccountId,
    endowed_accounts: Option<Vec<AccountId>>,
    enable_println: bool,
) -> GenesisConfig {
    let endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(|| {
        vec![
            get_account_id_from_seed::<sr25519::Public>("Alice"),
            get_account_id_from_seed::<sr25519::Public>("Bob"),
            get_account_id_from_seed::<sr25519::Public>("Charlie"),
            get_account_id_from_seed::<sr25519::Public>("Dave"),
            get_account_id_from_seed::<sr25519::Public>("Eve"),
            get_account_id_from_seed::<sr25519::Public>("Ferdie"),
            get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
            get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
            get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
            get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
            get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
            get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
        ]
    });
    let num_endowed_accounts = endowed_accounts.len();

    const ENDOWMENT: Balance = 8_000_000 * DOLLARS;
    const STASH: Balance = 500_000 * DOLLARS;
    const MINT: Balance = 2_000_000 * DOLLARS;

    GenesisConfig {
        frame_system: Some(SystemConfig {
            code: wasm_binary_unwrap().to_vec(),
            changes_trie_config: Default::default(),
        }),
        pallet_balances: Some(BalancesConfig {
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|k| (k, ENDOWMENT))
                .chain(initial_authorities.iter().map(|x| (x.0.clone(), STASH)))
                .collect(),
        }),
        pallet_indices: Some(IndicesConfig { indices: vec![] }),
        pallet_session: Some(SessionConfig {
            keys: initial_authorities
                .iter()
                .map(|x| {
                    (
                        x.0.clone(),
                        x.0.clone(),
                        session_keys(x.2.clone(), x.3.clone(), x.4.clone(), x.5.clone()),
                    )
                })
                .collect::<Vec<_>>(),
        }),
        pallet_staking: Some(StakingConfig {
            validator_count: initial_authorities.len() as u32 * 2,
            minimum_validator_count: initial_authorities.len() as u32,
            stakers: initial_authorities
                .iter()
                .map(|x| (x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator))
                .collect(),
            invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
            slash_reward_fraction: Perbill::from_percent(10),
            ..Default::default()
        }),
        pallet_democracy: Some(DemocracyConfig::default()),
        pallet_elections_phragmen: Some(ElectionsConfig {
            members: endowed_accounts
                .iter()
                .take((num_endowed_accounts + 1) / 2)
                .cloned()
                .map(|member| (member, STASH))
                .collect(),
        }),
        pallet_collective_Instance1: Some(CouncilConfig::default()),
        pallet_collective_Instance2: Some(TechnicalCommitteeConfig {
            members: endowed_accounts
                .iter()
                .take((num_endowed_accounts + 1) / 2)
                .cloned()
                .collect(),
            phantom: Default::default(),
        }),
        pallet_contracts: Some(ContractsConfig {
            current_schedule: pallet_contracts::Schedule {
                enable_println, // this should only be enabled on development chains
                ..Default::default()
            },
        }),
        pallet_sudo: Some(SudoConfig { key: root_key }),
        pallet_babe: Some(BabeConfig {
            authorities: vec![],
        }),
        pallet_im_online: Some(ImOnlineConfig { keys: vec![] }),
        pallet_authority_discovery: Some(AuthorityDiscoveryConfig { keys: vec![] }),
        pallet_grandpa: Some(GrandpaConfig {
            authorities: vec![],
        }),
        pallet_membership_Instance1: Some(Default::default()),
        pallet_treasury: Some(Default::default()),
        pallet_society: Some(SocietyConfig {
            members: endowed_accounts
                .iter()
                .take((num_endowed_accounts + 1) / 2)
                .cloned()
                .collect(),
            pot: 0,
            max_members: 999,
        }),
        pallet_vesting: Some(Default::default()),
        pallet_dummy: Some(DummyConfig { balance: MINT }),
    }
}

fn development_config_genesis() -> GenesisConfig {
    testnet_genesis(
        vec![authority_keys_from_seed("Alice")],
        get_account_id_from_seed::<sr25519::Public>("Alice"),
        None,
        true,
    )
}

/// Development config (single validator Alice)
pub fn development_config() -> ChainSpec {
    ChainSpec::from_genesis(
        "Development",
        "dev",
        ChainType::Development,
        development_config_genesis,
        vec![],
        None,
        None,
        None,
        Default::default(),
    )
}

fn local_testnet_genesis() -> GenesisConfig {
    testnet_genesis(
        vec![
            authority_keys_from_seed("Alice"),
            authority_keys_from_seed("Bob"),
        ],
        get_account_id_from_seed::<sr25519::Public>("Alice"),
        None,
        false,
    )
}

/// Local testnet config (multivalidator Alice + Bob)
pub fn local_testnet_config() -> ChainSpec {
    ChainSpec::from_genesis(
        "Local Testnet",
        "local_testnet",
        ChainType::Local,
        local_testnet_genesis,
        vec![],
        None,
        None,
        None,
        Default::default(),
    )
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::service::{new_full_base, new_light_base, NewFullBase};
    use sp_runtime::BuildStorage;

    fn local_testnet_genesis_instant_single() -> GenesisConfig {
        testnet_genesis(
            vec![authority_keys_from_seed("Alice")],
            get_account_id_from_seed::<sr25519::Public>("Alice"),
            None,
            false,
        )
    }

    /// Local testnet config (single validator - Alice)
    pub fn integration_test_config_with_single_authority() -> ChainSpec {
        ChainSpec::from_genesis(
            "Integration Test",
            "test",
            ChainType::Development,
            local_testnet_genesis_instant_single,
            vec![],
            None,
            None,
            None,
            Default::default(),
        )
    }

    /// Local testnet config (multivalidator Alice + Bob)
    pub fn integration_test_config_with_two_authorities() -> ChainSpec {
        ChainSpec::from_genesis(
            "Integration Test",
            "test",
            ChainType::Development,
            local_testnet_genesis,
            vec![],
            None,
            None,
            None,
            Default::default(),
        )
    }

    #[test]
    #[ignore]
    fn test_connectivity() {
        sc_service_test::connectivity(
            integration_test_config_with_two_authorities(),
            |config| {
                let NewFullBase {
                    task_manager,
                    client,
                    network,
                    transaction_pool,
                    ..
                } = new_full_base(config, |_, _| ())?;
                Ok(sc_service_test::TestNetComponents::new(
                    task_manager,
                    client,
                    network,
                    transaction_pool,
                ))
            },
            |config| {
                let (keep_alive, _, client, network, transaction_pool) = new_light_base(config)?;
                Ok(sc_service_test::TestNetComponents::new(
                    keep_alive,
                    client,
                    network,
                    transaction_pool,
                ))
            },
        );
    }

    #[test]
    fn test_create_development_chain_spec() {
        development_config().build_storage().unwrap();
    }

    #[test]
    fn test_create_local_testnet_chain_spec() {
        local_testnet_config().build_storage().unwrap();
    }

    #[test]
    fn test_staging_test_net_chain_spec() {
        staging_testnet_config().build_storage().unwrap();
    }
}
