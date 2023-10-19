use regex::Regex;
use crate::network::Network;

pub struct UpgradeHelper {
    pub network: Network,
    pub target_version: String,
}

impl UpgradeHelper {
    /// Creates a new instance of the upgrade helper.
    /// 
    /// # Arguments
    /// 
    /// * `network` - The network type used.
    /// * `target_version` - The target version to upgrade to.
    pub fn new(network: Network, target_version: String) -> UpgradeHelper {
        UpgradeHelper {
            network,
            target_version,
        }
    }

    /// Returns a boolean value if the defined target version fits 
    /// the requirements for the selected network type.
    /// The target version must be in the format `vX.Y.Z`.
    /// Testnet upgrades must use a release candidate with the suffix `-rcX`. 
    pub fn check_target_version(&self) -> bool {
        let re: Regex;

        match self.network {
            Network::LocalNode => {
                re = Regex::new(r"^v\d+\.\d{1}\.\d+(-rc\d+)*$").unwrap();
            },
            Network::Testnet => {
                re = Regex::new(r"^v\d+\.\d{1}\.\d+-rc\d+$").unwrap();
            },
            Network::Mainnet => {
                re = Regex::new(r"^v\d+\.\d{1}\.\d+$").unwrap();
            },
        }

        let valid_version = re.is_match(&self.target_version);
        if valid_version { 
            true 
        } else { 
            println!("Invalid target version for {}: {}", self.network, self.target_version); // TODO: return error here instead
            false 
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::network;

    #[test]
    fn test_check_target_version_local_node_pass() {
        let helper = UpgradeHelper::new( 
            network::Network::LocalNode, 
            "v14.0.0".to_string(),
        );
        assert_eq!(helper.check_target_version(), true);
    }

    #[test]
    fn test_check_target_version_local_node_fail() {
        let helper = UpgradeHelper::new( 
            network::Network::LocalNode, 
            "v14.0".to_string() ,
        );
        assert_eq!(helper.check_target_version(), false);
    }

    #[test]
    fn test_check_target_version_testnet_pass() {
        let helper = UpgradeHelper::new(
            network::Network::Testnet,
            "v14.0.0-rc1".to_string(),
        );
        assert_eq!(helper.check_target_version(), true);
    }

    #[test]
    fn test_check_target_version_testnet_fail() {
        let helper = UpgradeHelper::new(
            network::Network::Testnet,
            "v14.0.0".to_string(),
        );
        assert_eq!(helper.check_target_version(), false);
    }

    #[test]
    fn test_check_target_version_mainnet_pass() {
        let helper = UpgradeHelper::new(
            network::Network::Mainnet,
            "v14.0.0".to_string(),
        );
        assert_eq!(helper.check_target_version(), true);
    }

    #[test]
    fn test_check_target_version_mainnet_fail() {
        let helper = UpgradeHelper::new(
            network::Network::Mainnet,
            "v14.0.0-rc1".to_string(),
        );
        assert_eq!(helper.check_target_version(), false);
    }
}