use crate::{network::Network, inputs, proposal, release, version};
use chrono::{DateTime, Duration, Utc};
use std::process;

pub struct UpgradeHelper {
    pub network: Network,
    pub previous_version: String,
    pub target_version: String,
    pub proposal_name: String,
    pub upgrade_time: DateTime<Utc>,
    pub voting_period: Duration,
}

impl UpgradeHelper {
    /// Creates a new instance of the upgrade helper.
    pub fn new(network: Network, previous_version: &str, target_version: &str, upgrade_time: DateTime<Utc>) -> UpgradeHelper {
        let proposal_name = format!("Evmos {} {} Upgrade", network, target_version);

        let voting_period: Duration;
        match network {
            Network::LocalNode => voting_period = Duration::hours(1),
            Network::Testnet => voting_period = Duration::hours(12),
            Network::Mainnet => voting_period = Duration::hours(120),
        }

        UpgradeHelper {
            network,
            previous_version: previous_version.to_string(),
            target_version: target_version.to_string(),
            proposal_name,
            voting_period,
            upgrade_time: upgrade_time,
        }
    }

    /// Validates the upgrade helper.
    pub fn validate(&self) {
        // Check if the target version is valid
        let valid_version = version::is_valid_target_version(self.network, self.target_version.as_str());
        if !valid_version {
            println!(
                "Invalid target version for {}: {}",
                self.network, self.target_version
            );
            process::exit(1);
        }

        // Check if the previous version is valid
        let valid_version = version::is_valid_version(self.previous_version.as_str());
        if !valid_version {
            println!("Invalid previous version: {}", self.previous_version);
            process::exit(1);
        }

        // Check if the upgrade time is valid
        let valid_time = inputs::is_valid_upgrade_time(self.upgrade_time);
        if !valid_time {
            println!("Invalid upgrade time: {}", self.upgrade_time);
            process::exit(1);
        }

        println!("Upgrade configuration is valid")
    }

    /// Runs the main logic of the upgrade helper.
    pub fn run(&self) {
        // Check if release was already created
        let release_exists = release::check_release_exists(self.target_version.as_str());
        println!("Release exists: {}", release_exists.unwrap());

        // Prepare proposal
        let proposal: String;
        let proposal_res = proposal::prepare_proposal(&self);
        match proposal_res {
            Ok(contents) => { proposal = contents; }
            Err(e) => {
                println!("Error preparing proposal: {}", e);
                process::exit(1);
            }
        }

        // Write proposal to file
        let write_res = proposal::write_proposal_to_file(
            proposal.as_str(),
            self.network,
            self.target_version.as_str(),
        );
        match write_res {
            Ok(_) => {}
            Err(e) => {
                println!("Error writing proposal to file: {}", e);
                process::exit(1);
            }
        }
    }
}

/// Returns the voting period duration based on the network.
pub fn get_voting_period(network: Network) -> Duration {
    match network {
        Network::LocalNode => Duration::hours(1),
        Network::Testnet => Duration::hours(12),
        Network::Mainnet => Duration::hours(120),
    }
}
