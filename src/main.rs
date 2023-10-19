mod helper;
mod inputs;
mod network;

use crate::helper::UpgradeHelper; // TODO: Remove this and move to struct?
use std::process;

fn main() {
    // Prompt the user for the necessary input
    let used_network = inputs::get_used_network();
    let target_version = inputs::get_target_version();

    // Create an instance of the helper
    let helper = helper::MyUpgradeHelper::new(
        used_network, target_version,
    );

    // Check the target version
    let valid_version = helper.check_target_version();
    if !valid_version {
        process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_target_version_local_node_pass() {
        let helper = helper::MyUpgradeHelper::new( 
            network::Network::LocalNode, 
            "v14.0.0".to_string(),
        );
        assert_eq!(helper.check_target_version(), true);
    }

    #[test]
    fn test_check_target_version_local_node_fail() {
        let helper = helper::MyUpgradeHelper::new( 
            network::Network::LocalNode, 
            "v14.0".to_string() ,
        );
        assert_eq!(helper.check_target_version(), false);
    }

    #[test]
    fn test_check_target_version_testnet_pass() {
        let helper = helper::MyUpgradeHelper::new(
            network::Network::LocalNode,
            "v14.0.0-rc1".to_string(),
        );
        assert_eq!(helper.check_target_version(), true);
    }

    #[test]
    fn test_check_target_version_testnet_fail() {
        let helper = helper::MyUpgradeHelper::new(
            network::Network::LocalNode,
            "v14.0.0".to_string(),
        );
        assert_eq!(helper.check_target_version(), false);
    }

    #[test]
    fn test_check_target_version_mainnet_pass() {
        let helper = helper::MyUpgradeHelper::new(
            network::Network::Mainnet,
            "v14.0.0".to_string(),
        );
        assert_eq!(helper.check_target_version(), true);
    }

    #[test]
    fn test_check_target_version_mainnet_fail() {
        let helper = helper::MyUpgradeHelper::new(
            network::Network::Mainnet,
            "v14.0.0-rc1".to_string(),
        );
        assert_eq!(helper.check_target_version(), false);
    }
}