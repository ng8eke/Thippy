// Copyright (C) 2018-2021 Annie Lai.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use thippy_runtime::{AccountId, AuraId, Signature, EXISTENTIAL_DEPOSIT};
use cumulus_primitives_core::ParaId;
use hex_literal::hex;
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};

/// The ID of this Parachain as registered on the Relay Chain.
///
/// TODO: When we register as a common good parachain this will need to change.
pub const PARA_ID: u32 = 1002;

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec = sc_service::GenericChainSpec<thippy_runtime::GenesisConfig, Extensions>;

/// The default XCM version to set in genesis config.
const SAFE_XCM_VERSION: u32 = xcm::prelude::XCM_VERSION;

/// Helper function to generate a crypto pair from seed
pub fn get_public_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// The extensions for the [`ChainSpec`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ChainSpecGroup, ChainSpecExtension)]
#[serde(deny_unknown_fields)]
pub struct Extensions {
	/// The relay chain of the Parachain.
	pub relay_chain: String,
	/// The id of the Parachain.
	pub para_id: u32,
}

impl Extensions {
	/// Try to get the extension from the given `ChainSpec`.
	pub fn try_get(chain_spec: &dyn sc_service::ChainSpec) -> Option<&Self> {
		sc_chain_spec::get_extension(chain_spec.extensions())
	}
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate collator keys from seed.
///
/// This function's return type must always match the session keys of the chain in tuple format.
pub fn get_collator_keys_from_seed(seed: &str) -> AuraId {
	get_public_from_seed::<AuraId>(seed)
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_public_from_seed::<TPublic>(seed)).into_account()
}

/// Generate the session keys from individual elements.
///
/// The input must be a tuple of individual keys (a single arg for now since we have just one key).
pub fn thippy_session_keys(keys: AuraId) -> thippy_runtime::SessionKeys {
	thippy_runtime::SessionKeys { aura: keys }
}

pub fn development_config() -> ChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "ROC".into());
	properties.insert("tokenDecimals".into(), 12.into());
	properties.insert("ss58Format".into(), 42.into());

	ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || {
			thippy_genesis(
				// initial collators.
				vec![
					(
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						get_collator_keys_from_seed("Alice"),
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Bob"),
						get_collator_keys_from_seed("Bob"),
					),
				],
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
				],
				PARA_ID.into(),
			)
		},
		Vec::new(),
		None,
		None,
		None,
		None,
		Extensions {
			relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
			para_id: PARA_ID,
		},
	)
}

pub fn local_testnet_config() -> ChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "ROC".into());
	properties.insert("tokenDecimals".into(), 12.into());
	properties.insert("ss58Format".into(), 42.into());

	ChainSpec::from_genesis(
		// Name
		"Thippy",
		// ID
		"thippy_local",
		ChainType::Local,
		move || {
			thippy_genesis(
				// initial collators.
				vec![
					(
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						get_collator_keys_from_seed("Alice"),
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Bob"),
						get_collator_keys_from_seed("Bob"),
					),
				],
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
				],
				PARA_ID.into(),
			)
		},
		// Bootnodes
		Vec::new(),
		// Telemetry
		None,
		// Protocol ID
		Some("thippy-local"),
		// Fork ID
		None,
		// Properties
		Some(properties),
		// Extensions
		Extensions {
			relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
			para_id: PARA_ID,
		},
	)
}

pub fn rococo_testnet_config() -> ChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "ROC".into());
	properties.insert("tokenDecimals".into(), 12.into());

	ChainSpec::from_genesis(
		// Name
		"Thippy",
		// ID
		"thippy_rococo",
		ChainType::Live,
		move || {
			thippy_genesis(
				// Initial collators.
				//
				// Generated with: ./thippy_setup.sh 4.
				//
				// Note that these are SR25519 Aura public keys.
				//
				// Ask DevOps team for the $SECRET 🤫.
				vec![
					// 5GKFbTTgrVS4Vz1UWWHPqMZQNFWZtqo7H2KpCDyYhEL3aS26
					(
						hex!["bc09354c12c054c8f6b3da208485eacec4ac648bad348895273b37bab5a0937c"]
							.into(),
						hex!["bc09354c12c054c8f6b3da208485eacec4ac648bad348895273b37bab5a0937c"]
							.unchecked_into(),
					),
					// 5EPRJHm2GpABVWcwnAujcrhnrjFZyDGd5TwKFzkBoGgdRyv2
					(
						hex!["66be63b7bcbfb91040e5248e2d1ceb822cf219c57848c5924ffa3a1f8e67ba72"]
							.into(),
						hex!["66be63b7bcbfb91040e5248e2d1ceb822cf219c57848c5924ffa3a1f8e67ba72"]
							.unchecked_into(),
					),
					// 5GH62vrJrVZxLREcHzm2PR5uTLAT5RQMJitoztCGyaP4o3uM
					(
						hex!["ba62886472a0a9f66b5e39f1469ce1c5b3d8cad6be39078daf16f111e89d1e44"]
							.into(),
						hex!["ba62886472a0a9f66b5e39f1469ce1c5b3d8cad6be39078daf16f111e89d1e44"]
							.unchecked_into(),
					),
					// 5FHfoJDLdjRYX5KXLRqMDYBbWrwHLMtti21uK4QByUoUAbJF
					(
						hex!["8e97f65cda001976311df9bed39e8d0c956089093e94a75ef76fe9347a0eda7b"]
							.into(),
						hex!["8e97f65cda001976311df9bed39e8d0c956089093e94a75ef76fe9347a0eda7b"]
							.unchecked_into(),
					),
				],
				// Warning: The configuration for a production chain should not contain
				// any endowed accounts here, otherwise it'll be minting extra native tokens
				// from the relay chain on the parachain.
				vec![
					// NOTE: Remove endowed accounts if deployed on other relay chains.
					// Endowed accounts
					hex!["baa78c7154c7f82d6d377177e20bcab65d327eca0086513f9964f5a0f6bdad56"].into(),
					// AccountId of an account which `ink-waterfall` uses for automated testing
					hex!["0e47e2344d523c3cc5c34394b0d58b9a4200e813a038e6c5a6163cc07d70b069"].into(),
				],
				PARA_ID.into(),
			)
		},
		// Bootnodes
		vec![
			"/ip4/34.90.191.237/tcp/30333/p2p/12D3KooWKg3Rpxcr9oJ8n6khoxpGKWztCZydtUZk2cojHqnfLrpj"
				.parse()
				.expect("MultiaddrWithPeerId"),
			"/ip4/35.204.68.28/tcp/30333/p2p/12D3KooWPEXYrz8tHU3nDtPoPw4V7ou5dzMEWSTuUj7vaWiYVAVh"
				.parse()
				.expect("MultiaddrWithPeerId"),
			"/ip4/34.90.139.15/tcp/30333/p2p/12D3KooWEVU8AFNary4nP4qEnEcwJaRuy59Wefekzdu9pKbnVEhk"
				.parse()
				.expect("MultiaddrWithPeerId"),
			"/ip4/35.204.99.97/tcp/30333/p2p/12D3KooWP6pV3ZmcXzGDjv8ZMgA6nZxfAKDxSz4VNiLx6vVCQgJX"
				.parse()
				.expect("MultiaddrWithPeerId"),
		],
		// Telemetry
		None,
		// Protocol ID
		Some("thippy-rococo"),
		// Fork ID
		None,
		// Properties
		Some(properties),
		// Extensions
		Extensions { relay_chain: "rococo".into(), para_id: PARA_ID },
	)
}

fn thippy_genesis(
	invulnerables: Vec<(AccountId, AuraId)>,
	endowed_accounts: Vec<AccountId>,
	id: ParaId,
) -> thippy_runtime::GenesisConfig {
	thippy_runtime::GenesisConfig {
		system: thippy_runtime::SystemConfig {
			code: thippy_runtime::WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
		},
		balances: thippy_runtime::BalancesConfig {
			balances: endowed_accounts.iter().cloned().map(|k| (k, 1 << 60)).collect(),
		},
		parachain_info: thippy_runtime::ParachainInfoConfig { parachain_id: id },
		collator_selection: thippy_runtime::CollatorSelectionConfig {
			invulnerables: invulnerables.iter().cloned().map(|(acc, _)| acc).collect(),
			candidacy_bond: EXISTENTIAL_DEPOSIT * 16,
			..Default::default()
		},
		session: thippy_runtime::SessionConfig {
			keys: invulnerables
				.into_iter()
				.map(|(acc, aura)| {
					(
						acc.clone(),               // account id
						acc,                       // validator id
						thippy_session_keys(aura), // session keys
					)
				})
				.collect(),
		},
		// no need to pass anything to aura, in fact it will panic if we do. Session will take care
		// of this.
		aura: Default::default(),
		aura_ext: Default::default(),
		parachain_system: Default::default(),
		polkadot_xcm: thippy_runtime::PolkadotXcmConfig {
			safe_xcm_version: Some(SAFE_XCM_VERSION),
		},
	}
}
