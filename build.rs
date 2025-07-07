fn main() {
    substrate_build_script_utils::generate_cargo_keys();
    // For the node/worker version check, make sure we always rebuild the node and binary workers
    // when the version changes.
    substrate_build_script_utils::rerun_if_git_head_changed();
}
