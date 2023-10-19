mod helper;
mod inputs;
mod network;

use std::process;

fn main() {
    // Prompt the user for the necessary input
    let used_network = inputs::get_used_network();
    let target_version = inputs::get_target_version();

    // Create an instance of the helper
    let upgrade_helper = helper::UpgradeHelper::new(
        used_network, target_version,
    );

    // Check the target version
    let valid_version = upgrade_helper.check_target_version();
    if !valid_version {
        process::exit(1);
    }

    // Check if release was already created
    let release_exists = helper::check_release_exists(upgrade_helper.target_version);
    println!("Release exists: {}", release_exists.unwrap())
}