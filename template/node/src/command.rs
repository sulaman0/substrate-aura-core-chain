use futures::TryFutureExt;
// Substrate
use sc_cli::{ChainSpec, SubstrateCli};
use sc_service::DatabaseSource;
// Frontier
use fc_db::kv::frontier_database_dir;

use crate::{
	chain_spec,
	cli::{Cli, Subcommand},
	service::{self, db_config_dir},
};


#[cfg(feature = "runtime-benchmarks")]
use crate::chain_spec::get_account_id_from_seed;

impl SubstrateCli for Cli {
	fn impl_name() -> String {
		"Frontier Node".into()
	}

	fn impl_version() -> String {
		env!("SUBSTRATE_CLI_IMPL_VERSION").into()
	}

	fn description() -> String {
		env!("CARGO_PKG_DESCRIPTION").into()
	}

	fn author() -> String {
		env!("CARGO_PKG_AUTHORS").into()
	}

	fn support_url() -> String {
		"support.anonymous.an".into()
	}

	fn copyright_start_year() -> i32 {
		2021
	}

	fn load_spec(&self, id: &str) -> Result<Box<dyn ChainSpec>, String> {
		Ok(match id {
			"dev" => {
				let enable_manual_seal = self.sealing.map(|_| true).unwrap_or_default();
				Box::new(chain_spec::development_config(enable_manual_seal))
			}
			"" | "local" => Box::new(chain_spec::local_testnet_config()),
			path => Box::new(chain_spec::ChainSpec::from_json_file(
				std::path::PathBuf::from(path),
			)?),
		})
	}
}



/// Parse and run command line arguments
pub fn run() -> sc_cli::Result<()> {
	let cli = Cli::from_args();


}
