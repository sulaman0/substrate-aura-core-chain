//! Contains code to setup the command invocations in [`super::command`] which would
//! otherwise bloat that module.

use std::{sync::Arc, time::Duration};

use scale_codec::Encode;
// Substrate
use sc_cli::Result;
use sc_client_api::BlockBackend;
use sp_core::{ecdsa, Pair};
use sp_inherents::{InherentData, InherentDataProvider};
use sp_runtime::{generic::Era, OpaqueExtrinsic, SaturatedConversion};
// Frontier
use fp_account::AccountId20;
use frontier_template_runtime::{self as runtime, AccountId, Balance, BalancesCall, SystemCall};

use crate::service::Client;

// Generates extrinsics for the `benchmark overhead` command.
///
/// Note: Should only be used for benchmarking.
pub struct RemarkBuilder {
    client: Arc<Client>,
}

impl RemarkBuilder {
    /// Creates a new [`Self`] from the given client.
    pub fn new(client: Arc<Client>) -> Self {
        Self { client }
    }
}

impl frame_benchmarking_cli::ExtrinsicBuilder for RemarkBuilder {
    fn pallet(&self) -> &str {
        "system"
    }

    fn extrinsic(&self) -> &str {
        "remark"
    }

    fn build(&self, nonce: u32) -> std::result::Result<OpaqueExtrinsic, &'static str> {
        let acc = ecdsa::Pair::from_string("//Bob", None).expect("static values are valid; qed");
        let extrinsic: OpaqueExtrinsic = create_benchmark_extrinsic(
            self.client.as_ref(),
            acc,
            SystemCall::remark { remark: vec![] }.into(),
            nonce,
        )
            .into();

        Ok(extrinsic)
    }
}

/// Generates `Balances::TransferKeepAlive` extrinsics for the benchmarks.
///
/// Note: Should only be used for benchmarking.
pub struct TransferKeepAliveBuilder {
    client: Arc<Client>,
    dest: AccountId,
    value: Balance,
}

impl TransferKeepAliveBuilder {
    /// Creates a new [`Self`] from the given client.
    pub fn new(client: Arc<Client>, dest: AccountId, value: Balance) -> Self {
        Self {
            client,
            dest,
            value,
        }
    }
}

impl frame_benchmarking_cli::ExtrinsicBuilder for TransferKeepAliveBuilder {
    fn pallet(&self) -> &str {
        "balances"
    }

    fn extrinsic(&self) -> &str {
        "transfer_keep_alive"
    }

    fn build(&self, nonce: u32) -> std::result::Result<OpaqueExtrinsic, &'static str> {
        let acc = ecdsa::Pair::from_string("//Bob", None).expect("static values are valid; qed");
        let extrinsic: OpaqueExtrinsic = create_benchmark_extrinsic(
            self.client.as_ref(),
            acc,
            BalancesCall::transfer_keep_alive {
                dest: self.dest,
                value: self.value,
            }
                .into(),
            nonce,
        )
            .into();

        Ok(extrinsic)
    }
}

/// Create a transaction using the given `call`.
///
/// Note: Should only be used for benchmarking.

