use color_eyre::eyre;
use polkadot_sdk::polkadot_cli::{run_doppelganger, Cli};
use polkadot_sdk::sc_cli::clap::Parser;

fn main()  -> eyre::Result<()>{
	color_eyre::install()?;
	let cli = Cli::try_parse()?;
	println!("{:?}", cli);
	run_doppelganger(cli)?;
	Ok(())
}