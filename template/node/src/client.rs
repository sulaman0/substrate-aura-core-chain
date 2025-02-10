use scale_codec::Codec;
// Substrate
use sc_executor::WasmExecutor;
use sp_runtime::traits::{Block as BlockT, MaybeDisplay};

use crate::eth::EthCompatRuntimeApiCollection;

/// Full backend.
pub type FullBackend<B> = sc_service::TFullBackend<B>;
/// Full client.
pub type FullClient<B, RA, HF> = sc_service::TFullClient<B, RA, WasmExecutor<HF>>;

/// A set of APIs that every runtime must implement.
