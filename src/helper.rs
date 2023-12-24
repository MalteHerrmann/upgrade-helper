use crate::{block::get_estimated_height, inputs, network::Network, version};
use chrono::{DateTime, Duration, Utc};
use std::path::{Path, PathBuf};
use std::{fs, process};

/// Contains all relevant information for the scheduled upgrade.
pub struct UpgradeHelper {
    /// The chain ID of the node.
    pub chain_id: String,
    /// The home directory of the node.
    pub home: PathBuf,
    /// The network to create the commands and proposal description for.
    pub network: Network,
    /// The previous version to upgrade from.
    pub previous_version: String,
    /// The name of the proposal.
    pub proposal_name: String,
    /// The name of the proposal file.
    pub proposal_file_name: String,
    /// The target version to upgrade to.
    pub target_version: String,
    /// The scheduled height of the upgrade.
    pub upgrade_height: u64,
    /// The scheduled time of the upgrade.
    pub upgrade_time: DateTime<Utc>,
    /// The voting period duration.
    pub voting_period: Duration,
}

impl UpgradeHelper {
    /// Creates a new instance of the upgrade helper.
    pub async fn new(
        network: Network,
        previous_version: &str,
        target_version: &str,
        upgrade_time: DateTime<Utc>,
    ) -> UpgradeHelper {
        let chain_id = get_chain_id(network);
        // TODO: Get from input eventually
        let home = get_home(network);

        let proposal_name = format!("Evmos {} {} Upgrade", network, target_version);
        let voting_period = get_voting_period(network);
        let upgrade_height = get_estimated_height(network, upgrade_time).await;
        println!("Estimated upgrade height: {}", upgrade_height);
        let proposal_file_name = format!("proposal-{}-{}.md", network, target_version);

        UpgradeHelper {
            chain_id,
            home,
            network,
            previous_version: previous_version.to_string(),
            proposal_name,
            proposal_file_name,
            target_version: target_version.to_string(),
            upgrade_height,
            upgrade_time,
            voting_period,
        }
    }

    /// Validates the upgrade helper.
    pub fn validate(&self) {
        // Check if the target version is valid
        let valid_version =
            version::is_valid_target_version(self.network, self.target_version.as_str());
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

        // Check if home folder exists
        let exists = path_exists(&self.home);
        if !exists {
            println!(
                "Home folder does not exist: {}",
                &self
                    .home
                    .to_str()
                    .expect("Failed to convert home path to string")
            );
            process::exit(1);
        }

        println!("Upgrade configuration is valid")
    }
}

/// Checks whether a given path exists.
fn path_exists(path: &Path) -> bool {
    let res = fs::metadata(path);
    match res {
        Ok(_) => {}
        Err(_) => {
            return false;
        }
    }

    let metadata = res.unwrap();
    if metadata.is_dir() || metadata.is_file() {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod path_tests {
    use super::*;

    #[test]
    fn test_path_exists() {
        let path = Path::new("/tmp");
        assert_eq!(path_exists(path), true);
    }

    #[test]
    fn test_path_does_not_exist() {
        let path = Path::new("/tmp/does-not-exist");
        assert_eq!(path_exists(path), false);
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

/// Returns the home directory based on the network.
fn get_home(network: Network) -> PathBuf {
    // home dir of user
    let mut user_home = dirs::home_dir().expect("Failed to get home directory");
    match network {
        Network::LocalNode => user_home.push(".tmp-evmosd"),
        Network::Testnet => user_home.push(".evmosd"),
        Network::Mainnet => user_home.push(".evmosd"),
    }
    user_home
}

/// Returns the chain ID based on the network.
fn get_chain_id(network: Network) -> String {
    match network {
        Network::LocalNode => "evmos_9000-4".to_string(),
        Network::Testnet => "evmos_9000-4".to_string(),
        Network::Mainnet => "evmos_9001-2".to_string(),
    }
}
