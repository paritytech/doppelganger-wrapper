use color_eyre::eyre;
use polkadot_sdk::polkadot_cli::{Cli, run,  run_doppelganger};
use polkadot_sdk::sc_cli::clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[allow(missing_docs)]
struct DoppelgangerCli {
    #[allow(missing_docs)]
    #[clap(flatten)]
    pub inner_cli: Cli,

    /// Path to json overrides
    #[arg(long, value_name = "PATH")]
    pub json_overrides: Option<PathBuf>,
}

fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    let cli = DoppelgangerCli::parse();
    if cli.inner_cli.subcommand.is_some() {
        run()?;
    } else {
        run_doppelganger(cli.inner_cli)?;
    }

    Ok(())
}
