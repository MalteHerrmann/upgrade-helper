mod helper;
mod inputs;
mod network;
mod proposal;
mod release;
mod version;

use chrono::{DateTime, Utc};
use helper::UpgradeHelper;
use std::process;

/// Creates a new instance of the upgrade helper based on querying the user for the necessary input.
fn get_helper_from_inputs() -> UpgradeHelper {
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
    let valid_version = version::is_valid_target_version(used_network, target_version.as_str());
    if !valid_version {
        println!(
            "Invalid target version for {}: {}",
            used_network, target_version
        );
        process::exit(1);
    }

    // Query the date and time for the upgrade
    let upgrade_time: DateTime<Utc>;
    let voting_period = helper::get_voting_period(used_network);
    let time_option = inputs::get_upgrade_date(voting_period, Utc::now());
    match time_option {
        Some(time) => {
            upgrade_time = time;
        }
        None => {
            process::exit(1);
        }
    }

    // Create an instance of the helper
    UpgradeHelper::new(
        used_network,
        previous_version.as_str(),
        target_version.as_str(),
        upgrade_time,
    )
}

fn main() {
    // Create an instance of the helper
    let upgrade_helper = get_helper_from_inputs();

    // Validate the helper configuration
    upgrade_helper.validate();

    // Run the main functionality of the helper.
    upgrade_helper.run();
}
