use std::{collections::BTreeMap, str::FromStr};

use hex_literal::hex;
// Substrate
use sc_chain_spec::{ChainType, Properties};
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
#[allow(unused_imports)]
use sp_core::ecdsa;
use sp_core::{Pair, Public, H160, U256};
use sp_runtime::traits::{IdentifyAccount, Verify};
// Frontier
use frontier_template_runtime::{AccountId, Balance, SS58Prefix, Signature, WASM_BINARY};

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

#[allow(dead_code)]
type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
/// For use with `AccountId32`, `dead_code` if `AccountId20`.

#[allow(dead_code)]
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
    (get_from_seed::<AuraId>(s), get_from_seed::<GrandpaId>(s))
}

fn properties() -> Properties {
    let mut properties = Properties::new();
    properties.insert("tokenDecimals".into(), 18.into());
    properties.insert("ss58Format".into(), SS58Prefix::get().into());
    properties
}

const UNITS: Balance = 1_000_000_000_000_000_000;
pub fn development_config(enable_manual_seal: bool) -> ChainSpec {
    ChainSpec::builder(WASM_BINARY.expect("WASM not available"), Default::default())
        .with_name("Development")
        .with_id("dev")
        .with_chain_type(ChainType::Development)
        .with_properties(properties())
        .with_genesis_config_patch(testnet_genesis(
            // Sudo account (Alith)
            AccountId::from(hex!("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac")),
            // Pre-funded accounts
            vec![
                AccountId::from(hex!("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac")), // Alith
                AccountId::from(hex!("3Cd0A705a2DC65e5b1E1205896BaA2be8A07c6e0")), // Baltathar
                AccountId::from(hex!("798d4Ba9baf0064Ec19eB4F0a1a45785ae9D6DFc")), // Charleth
                AccountId::from(hex!("773539d4Ac0e786233D90A233654ccEE26a613D9")), // Dorothy
                AccountId::from(hex!("Ff64d3F6efE2317EE2807d223a0Bdc4c0c49dfDB")), // Ethan
                AccountId::from(hex!("C0F0f4ab324C46e55D02D0033343B4Be8A55532d")), // Faith
            ],
            // Initial PoA authorities
            vec![authority_keys_from_seed("Alice")],
            // Ethereum chain ID
            SS58Prefix::get() as u64,
            enable_manual_seal,
        ))
        .build()
}