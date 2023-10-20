mod helper;
mod inputs;
mod network;
mod release;
mod proposal;

use std::process;

fn main() {
    // Prompt the user for the necessary input
    let used_network = inputs::get_used_network();
    let target_version = inputs::get_target_version();

    // Create an instance of the helper
    let upgrade_helper = helper::UpgradeHelper::new(
        used_network, target_version.as_str(),
    );

    // Check the target version
    let valid_version = upgrade_helper.check_target_version();
    if !valid_version {
        process::exit(1);
    }

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
        Ok(proposal) => println!("{}", proposal),
        Err(err) => {
            println!("Error: {}", err);
            process::exit(1);
        },
    }
}
