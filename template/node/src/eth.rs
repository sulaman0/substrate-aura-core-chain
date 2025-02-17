use std::{
	collections::BTreeMap,
	path::PathBuf,
	sync::{Arc, Mutex},
	time::Duration,
};

use futures::{future, prelude::*};
// Substrate
use sc_client_api::BlockchainEvents;
use sc_executor::HostFunctions;
use sc_network_sync::SyncingService;
use sc_service::{error::Error as ServiceError, Configuration, TaskManager};
use sp_api::ConstructRuntimeApi;
use sp_core::H256;
use sp_runtime::traits::Block as BlockT;
// Frontier
pub use fc_consensus::FrontierBlockImport;
use fc_rpc::EthTask;
pub use fc_rpc_core::types::{FeeHistoryCache, FeeHistoryCacheLimit, FilterPool};
pub use fc_storage::{StorageOverride, StorageOverrideHandler};
use crate::client::{FullBackend, FullClient};
/// Frontier DB backend type.
pub type FrontierBackend<B, C> = fc_db::Backend<B, C>;
pub fn db_config_dir(config: &Configuration) -> PathBuf {
	config.base_path.config_dir(config.chain_spec.id())
}
/// Available frontier backend types.
#[derive(Debug, Copy, Clone, Default, clap::ValueEnum)]
pub enum BackendType {
	/// Either RocksDb or ParityDb as per inherited from the global backend settings.
	#[default]
	KeyValue,
	/// Sql database with custom log indexing.
	Sql,
}
/// The ethereum-compatibility configuration used to run a node.
#[derive(Clone, Debug, clap::Parser)]
pub struct EthConfiguration {
/// Maximum number of logs in a query.
	#[arg(long, default_value = "10000")]
	pub max_past_logs: u32,

/// Maximum fee history cache size.
	#[arg(long, default_value = "2048")]
pub fee_history_limit: u64,

	#[arg(long)]
	pub enable_dev_signer: bool,

/// The dynamic-fee pallet target gas price set by block author
	#[arg(long, default_value = "1")]
pub target_gas_price: u64,

/// Maximum allowed gas limit will be `block.gas_limit * execute_gas_limit_multiplier`
/// when using eth_call/eth_estimateGas.
	#[arg(long, default_value = "10")]
	pub execute_gas_limit_multiplier: u64,
