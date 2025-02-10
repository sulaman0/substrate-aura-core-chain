use crate::service::EthConfiguration;

/// Available Sealing methods.
#[derive(Copy, Clone, Debug, Default, clap::ValueEnum)]
pub enum Sealing {
	/// Seal using rpc method.
	#[default]
	Manual,
	/// Seal when transaction is executed.
	Instant,
}


