//! Execute worker.

use polkadot_sdk::polkadot_node_core_pvf_common;
use polkadot_sdk::polkadot_node_core_pvf_execute_worker;
use polkadot_sdk::polkadot_cli;

polkadot_node_core_pvf_common::decl_worker_main!(
	"execute-worker",
	polkadot_node_core_pvf_execute_worker::worker_entrypoint,
	polkadot_cli::NODE_VERSION,
	env!("SUBSTRATE_CLI_COMMIT_HASH"),
);