//! Prepare worker.

use polkadot_sdk::polkadot_node_core_pvf_common;
use polkadot_sdk::polkadot_node_core_pvf_prepare_worker;
use polkadot_sdk::polkadot_cli;

polkadot_node_core_pvf_common::decl_worker_main!(
	"prepare-worker",
	polkadot_node_core_pvf_prepare_worker::worker_entrypoint,
	polkadot_cli::NODE_VERSION,
	env!("SUBSTRATE_CLI_COMMIT_HASH"),
);