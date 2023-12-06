mod block;
mod command;
mod helper;
mod http;
mod inputs;
mod network;
mod proposal;
mod release;
mod version;

use chrono::Utc;
use helper::UpgradeHelper;
use std::process;

/// Creates a new instance of the upgrade helper based on querying the user for the necessary input.
async fn get_helper_from_inputs() -> UpgradeHelper {
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
    let voting_period = helper::get_voting_period(used_network);
    let upgrade_time = match inputs::get_upgrade_date(voting_period, Utc::now()) {
        Some(time) => time,
        None => {
            process::exit(1);
        }
    };

    // Create an instance of the helper
    UpgradeHelper::new(
        used_network,
        previous_version.as_str(),
        target_version.as_str(),
        upgrade_time,
    )
    .await
}

/// Runs the logic to prepare the proposal description and write
/// it to file.
async fn run_proposal_preparation(helper: &UpgradeHelper) {
    let description = proposal::prepare_proposal(helper).expect("failed to prepare proposal");
    proposal::write_proposal_to_file(&description, &helper.proposal_file_name)
        .expect("failed to write proposal to file");
}

/// Runs the logic to prepare the command to submit the proposal.
async fn run_command_preparation(helper: &UpgradeHelper) {
    // Check if release was already created
    let release_exists = release::check_release_exists(helper.target_version.as_str()).await;
    if !release_exists {
        println!("Release {} does not exist yet.", helper.target_version);
        process::exit(1);
    }

    // Prepare command to submit proposal
    let command = match command::prepare_command(&helper).await {
        Ok(contents) => contents,
        Err(e) => {
            println!("Error preparing command: {}", e);
            process::exit(1);
        }
    };

    print!("Command: {}\n\n", command);
}

#[tokio::main]
async fn main() {
    // if the first argument is "proposal", run the proposal command
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 {
        match args[1].as_str() {
            "proposal" => {
                // Create an instance of the helper
                let upgrade_helper = get_helper_from_inputs().await;

                // Validate the helper configuration
                upgrade_helper.validate();

                // Run the main functionality of the helper.
                run_proposal_preparation(&upgrade_helper).await;
            }
            "command" => {
                // Create an instance of the helper
                let upgrade_helper = get_helper_from_inputs().await;
                // Validate the helper configuration
                upgrade_helper.validate();
                // Run the main functionality of the helper.
                run_command_preparation(&upgrade_helper).await;
            }
            _ => {
                println!(
                    "Possible usage:\n  - upgrade-helper proposal\n  - upgrade-helper command\n"
                )
            }
        }
    } else {
        println!("Possible usage:\n  - upgrade-helper proposal\n  - upgrade-helper command\n")
    }
}
