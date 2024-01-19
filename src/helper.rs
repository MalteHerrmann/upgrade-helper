use crate::{block::get_estimated_height, inputs, network::Network, version};
use chrono::{DateTime, Duration, Utc};
use std::path::{Path, PathBuf};
use std::{fs, io};

/// Contains all relevant information for the scheduled upgrade.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct UpgradeHelper {
    /// The chain ID of the node.
    pub chain_id: String,
    /// The name of the config file.
    pub config_file_name: String,
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
    /// The number of hours for the voting period.
    pub voting_period: i64,
}

impl UpgradeHelper {
    /// Creates a new instance of the upgrade helper.
    ///
    /// TODO: check errors instead of using expect everywhere
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
        // TODO: Move out to input and pass values into function, helper should be instantiated with all values
        let upgrade_height = get_estimated_height(network, upgrade_time).await;
        let proposal_file_name = format!("proposal-{}-{}.md", network, target_version);
        let config_file_name = format!("proposal-{}-{}.json", network, target_version);

        UpgradeHelper {
            chain_id,
            config_file_name,
            home,
            network,
            previous_version: previous_version.to_string(),
            proposal_name,
            proposal_file_name,
            target_version: target_version.to_string(),
            upgrade_height,
            upgrade_time,
            voting_period: voting_period.num_hours(),
        }
    }

    /// Validates the upgrade helper.
    pub fn validate(&self) -> Result<(), String> {
        // Check if the target version is valid
        let valid_version =
            version::is_valid_target_version(self.network, self.target_version.as_str());
        if !valid_version {
            return Err(format!(
                "Invalid target version for {}: {}",
                self.network, self.target_version
            ));
        }

        // Check if the previous version is valid
        let valid_version = version::is_valid_version(self.previous_version.as_str());
        if !valid_version {
            return Err(format!(
                "Invalid previous version: {}",
                self.previous_version
            ));
        }

        // Check if the upgrade time is valid
        let valid_time = inputs::is_valid_upgrade_time(self.upgrade_time);
        if !valid_time {
            return Err(format!("Invalid upgrade time: {}", self.upgrade_time));
        }

        // Check if home folder exists
        let exists = path_exists(&self.home);
        if !exists {
            return Err(format!(
                "Home folder does not exist: {}",
                &self
                    .home
                    .to_str()
                    .expect("Failed to convert home path to string")
            ));
        }

        Ok(())
    }

    /// Exports the upgrade helper to a JSON file.
    pub fn write_to_json(&self) -> io::Result<()> {
        let json = serde_json::to_string_pretty(&self).expect("Failed to convert to JSON");
        let path = Path::new(&self.config_file_name);

        fs::write(&path, json)
    }
}

/// Returns the upgrade helper from a JSON file.
pub fn from_json(path: &Path) -> Result<UpgradeHelper, String> {
    let json = match fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(e) => {
            return Err(format!(
                "Failed to read contents of file '{}': {}",
                path.to_str().unwrap(),
                e
            ));
        }
    };

    match serde_json::from_str(&json) {
        Ok(helper) => Ok(helper),
        Err(e) => Err(format!(
            "Failed to convert contents of file '{}' to JSON: {}",
            path.to_str().unwrap(),
            e
        )),
    }
}

#[cfg(test)]
mod helper_tests {
    use super::*;
    use chrono::TimeZone;

    #[tokio::test]
    async fn test_new_upgrade_helper() {
        let network = Network::Testnet;
        let previous_version = "v14.0.0";
        let target_version = "v14.0.0-rc1";
        let upgrade_time = Utc.with_ymd_and_hms(2021, 1, 1, 0, 0, 0).unwrap();
        let helper =
            UpgradeHelper::new(network, previous_version, target_version, upgrade_time).await;
        assert_eq!(helper.chain_id, "evmos_9000-4");
        assert_eq!(helper.config_file_name, "proposal-Testnet-v14.0.0-rc1.json");
        assert!(
            helper.home.to_str().unwrap().contains(".evmosd"),
            "expected different home directory"
        );
        assert_eq!(helper.network, Network::Testnet);
        assert_eq!(helper.previous_version, "v14.0.0");
        assert_eq!(helper.proposal_name, "Evmos Testnet v14.0.0-rc1 Upgrade");
        assert_eq!(helper.proposal_file_name, "proposal-Testnet-v14.0.0-rc1.md");
        assert_eq!(helper.target_version, "v14.0.0-rc1");
    }

    #[tokio::test]
    async fn test_write_to_json_and_read_from_json() {
        let helper = UpgradeHelper::new(
            Network::Testnet,
            "v14.0.0",
            "v14.0.0-rc1",
            Utc.with_ymd_and_hms(2021, 1, 1, 0, 0, 0).unwrap(),
        )
        .await;

        assert!(
            helper.write_to_json().is_ok(),
            "expected success writing helper information to JSON file"
        );

        // assert that the config file exists
        let path = Path::new(&helper.config_file_name);
        assert!(path_exists(path), "expected config file to exist");

        let read_input_helper = from_json(path).expect("failed to read helper from JSON file");
        assert_eq!(helper.chain_id, read_input_helper.chain_id);
        assert_eq!(helper.config_file_name, read_input_helper.config_file_name);

        // remove the config file
        match fs::remove_file(&path) {
            Ok(_) => {}
            Err(e) => {
                println!("Failed to remove file '{}': {}", path.to_str().unwrap(), e);
                assert!(false, "expected success removing config file");
            }
        }
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
