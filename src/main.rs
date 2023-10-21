mod helper;
mod inputs;
mod network;
mod release;
mod proposal;
mod version;

use std::process;

fn main() {
    // Query and check the network to use
    let used_network = inputs::get_used_network();

    // Query and check the version to upgrade from
    let previous_version = inputs::get_text("Previous version to upgrade from:");
    let valid_version = version::is_valid_version(previous_version.as_str());
    if !valid_version {
        println!("Invalid previous version: {}", previous_version);
        process::exit(1);
    }

    // Query and check the target version to upgrade to
    let target_version = inputs::get_text("Target version to upgrade to:");
    let valid_version = version::is_valid_target_version(
        used_network, target_version.as_str(),
    );
    if !valid_version {
        println!("Invalid target version for {}: {}", used_network, target_version);
        process::exit(1);
    }

    // Create an instance of the helper
    let upgrade_helper = helper::UpgradeHelper::new(
        used_network,
        previous_version.as_str(),
        target_version.as_str(),
    );

    // Check if release was already created
    let release_exists = release::check_release_exists(upgrade_helper.target_version.as_str());
    println!("Release exists: {}", release_exists.unwrap());

    // Prepare proposal
    let proposal_res = proposal::prepare_proposal(
        upgrade_helper.network,
        upgrade_helper.target_version.as_str(),
        "v0.0.1", // TODO: get previous version from proposals?
    );
    match proposal_res {
        Err(err) => {
            println!("Error: {}", err);
            process::exit(1);
        },
        _ => {},
    }

    // Write proposal to file
    let write_res = proposal::write_proposal_to_file(
        proposal_res.unwrap().as_str(),
        upgrade_helper.network,
        upgrade_helper.target_version.as_str(),
    );
    match write_res {
        Err(err) => {
            println!("Error: {}", err);
            process::exit(1);
        },
        _ => {},
    }
}
