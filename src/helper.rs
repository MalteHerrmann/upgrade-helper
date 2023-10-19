use regex::Regex;
use crate::network::Network;

// Trait to define the interface
pub trait UpgradeHelper {
    const DEFAULT_HOME: &'static str;
    fn check_target_version(&self) -> bool;
}

pub struct MyUpgradeHelper {
    network: Network,
    target_version: String,
}

impl MyUpgradeHelper {
    /// Creates a new instance of the upgrade helper.
    /// 
    /// # Arguments
    /// 
    /// * `network` - The network type used.
    /// * `target_version` - The target version to upgrade to.
    pub fn new(network: Network, target_version: String) -> MyUpgradeHelper {
        MyUpgradeHelper {
            network,
            target_version,
        }
    }
}

impl UpgradeHelper for MyUpgradeHelper {
    const DEFAULT_HOME: &'static str = "/path/to/default/home";

    /// Returns a boolean value if the defined target version fits 
    /// the requirements for the selected network type.
    /// The target version must be in the format `vX.Y.Z`.
    /// Testnet upgrades must use a release candidate with the suffix `-rcX`. 
    fn check_target_version(&self) -> bool {
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
            println!("Invalid target version for {}: {}", self.network, self.target_version);
            false 
        }
    }
}
