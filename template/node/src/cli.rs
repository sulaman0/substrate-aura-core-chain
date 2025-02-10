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

#[derive(Debug, clap::Parser)]
pub struct Cli {
	#[command(subcommand)]
	pub subcommand: Option<Subcommand>,

	#[allow(missing_docs)]
	#[command(flatten)]
	pub run: sc_cli::RunCmd,

	/// Choose sealing method.
	#[arg(long, value_enum, ignore_case = true)]
	pub sealing: Option<Sealing>,

	#[command(flatten)]
	pub eth: EthConfiguration,
}



