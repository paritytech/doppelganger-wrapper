#![warn(missing_docs)]
#![warn(unused_extern_crates)]

//! Doppelganger parachain node.

use polkadot_sdk::doppelganger_lib::chain_spec::{ChainSpecLoader, RuntimeResolver};
use polkadot_sdk::polkadot_omni_node_lib::{CliConfig as CliConfigT, RunConfig, run};

struct CliConfig;

impl CliConfigT for CliConfig {
    fn impl_version() -> String {
        "Doppelganger-parachain".into()
    }

    fn author() -> String {
        env!("CARGO_PKG_AUTHORS").into()
    }

    fn support_url() -> String {
        "https://github.com/paritytech/doppelganger-wrapper/issues/new".into()
    }

    fn copyright_start_year() -> u16 {
        2025
    }
}

fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;

    let config = RunConfig {
        chain_spec_loader: Box::new(ChainSpecLoader),
        runtime_resolver: Box::new(RuntimeResolver),
    };
    Ok(run::<CliConfig>(config)?)
}
