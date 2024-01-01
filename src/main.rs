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
async fn get_helper_from_inputs() -> Result<UpgradeHelper, String> {
    // Query and check the network to use
    let used_network = inputs::get_used_network()?;

    // Query and check the version to upgrade from
    let previous_version = inputs::get_text("Previous version to upgrade from:");
    let valid_version = version::is_valid_version(previous_version.as_str());
    if !valid_version {
        return Err(format!("Invalid previous version: {}", previous_version));
    }

    // Query and check the target version to upgrade to
    let target_version = inputs::get_text("Target version to upgrade to:");
    let valid_version = version::is_valid_target_version(used_network, target_version.as_str());
    if !valid_version {
        return Err(format!(
            "Invalid target version for {}: {}",
            used_network, target_version
        ));
    }

    // Query the date and time for the upgrade
    let voting_period = helper::get_voting_period(used_network);
    let upgrade_time = inputs::get_upgrade_date(voting_period, Utc::now())?;

    // Create an instance of the helper
    Ok(UpgradeHelper::new(
        used_network,
        previous_version.as_str(),
        target_version.as_str(),
        upgrade_time,
    )
    .await)
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
            println!("Error preparing proposal command: {}", e);
            process::exit(1);
        }
    };

    print!("Command: \n{}\n\n", command);
}

#[tokio::main]
async fn main() {
    // if the first argument is "proposal", run the proposal command
    let args: Vec<String> = std::env::args().collect();
    // TODO: add help command
    // TODO: use some CLI package instead of this manual parsing
    if args.len() == 2 {
        match args[1].as_str() {
            "generate-proposal" => {
                // Create an instance of the helper
                let upgrade_helper = match get_helper_from_inputs().await {
                    Ok(helper) => helper,
                    Err(e) => {
                        println!("Error creating helper: {}", e);
                        process::exit(1);
                    }
                };

                // Validate the helper configuration
                match upgrade_helper.validate() {
                    Ok(_) => {}
                    Err(e) => {
                        println!("Invalid configuration: {}", e);
                        process::exit(1);
                    }
                };

                // Export the configuration
                match upgrade_helper.write_to_json() {
                    Ok(_) => {
                        println!(
                            "successfully wrote config to json: {}",
                            &upgrade_helper.config_file_name
                        )
                    }
                    Err(e) => {
                        println!(
                            "failed to write config to {}: {}",
                            &upgrade_helper.config_file_name, e
                        );
                    }
                }

                // Run the main functionality of the helper.
                run_proposal_preparation(&upgrade_helper).await;
            }
            "generate-command" => {
                // Choose configuration from all found configurations in folder
                let config = match inputs::choose_config() {
                    Ok(config) => config,
                    Err(e) => {
                        println!("Error choosing config: {}", e);
                        process::exit(1);
                    }
                };

                // Read the helper configuration
                let upgrade_helper = match helper::from_json(config.as_path()) {
                    Ok(helper) => helper,
                    Err(e) => {
                        println!("Error reading config: {}", e);
                        process::exit(1);
                    }
                };

                // Validate the helper configuration
                match upgrade_helper.validate() {
                    Ok(_) => {}
                    Err(e) => {
                        println!("Invalid configuration: {}", e);
                        process::exit(1);
                    }
                };

                // Run the main functionality of the helper.
                run_command_preparation(&upgrade_helper).await;
            }
            _ => {
                println!(
                    "Possible usage:\n  - upgrade-helper generate-proposal\n  - upgrade-helper generate-command\n"
                )
            }
        }
    } else {
        println!(
            "Possible usage:\n  - upgrade-helper generate-proposal\n  - upgrade-helper generate-command\n"
        )
    }
}
