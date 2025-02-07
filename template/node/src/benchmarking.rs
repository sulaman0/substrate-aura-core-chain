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

