use regex::Regex;
use std::process;

use inquire::Select;

// Enum to represent different network options
#[derive(Debug)]
enum Network {
    LocalNode,
    Testnet,
    Mainnet,
}

// Trait to define the interface
trait UpgradeHelper {
    const DEFAULT_HOME: &'static str;
    fn check_target_version(&self);
}

struct MyUpgradeHelper {
    network: Network,
    target_version: String,
}

impl UpgradeHelper for MyUpgradeHelper {
    const DEFAULT_HOME: &'static str = "/path/to/default/home";

    /// Returns a boolean value if the defined target version fits 
    /// the requirements for the selected network type.
    /// The target version must be in the format `vX.Y.Z`.
    /// Testnet upgrades must use a release candidate with the suffix `-rcX`. 
    fn check_target_version(&self) {
        let re: Regex;

        match self.network {
            Network::LocalNode => {
                re = Regex::new(r"v\d+\.\d{1}\.\d+(-rc\d+)*").unwrap();
            },
            Network::Testnet => {
                re = Regex::new(r"v\d+\.\d{1}\.\d+-rc\d+").unwrap();
            },
            Network::Mainnet => {
                re = Regex::new(r"v\d+\.\d{1}\.\d+").unwrap();
            },
        }

        let valid_version = re.is_match(&self.target_version);
        if !valid_version {
            println!("Invalid target version: {}", self.target_version);
            process::exit(1);
        }
    }
}

/// Prompts the user to select the network type used.
fn get_used_network() -> Network {
    let used_network: Network;

    let network_options = vec![
        "Local Node",
        "Testnet",
        "Mainnet",
    ];

    // Prompt the user to select the network
    let chosen_network = Select::new("Select network", network_options).prompt();
    match chosen_network {
        Ok(choice) => {
            match choice {
                "Local Node" => used_network = Network::LocalNode,
                "Testnet" => used_network = Network::Testnet,
                "Mainnet" => used_network = Network::Mainnet,
                &_ => {
                    println!("Invalid network selected: {:?}", choice);
                    process::exit(1);
                },
            }
        }
        Err(e) => { 
            println!("Error selecting network: {}", e); 
            process::exit(1);
        }
    }

    used_network
}

fn main() {
    // Initialize the network variable
    let used_network = get_used_network();

    let target_version: String;
    // Prompt the user to input the desired target version
    let chosen_target_version = inquire::Text::new("Target version to upgrade to:")
        .prompt();
    match chosen_target_version {
        Ok(version) => {
            println!("Target version: {}", version);
            // Initialize the target version variable
            target_version = version;
        }
        Err(e) => { 
            println!("Error selecting target version: {}", e); 
            process::exit(1);
        }
    }


    // Create an instance of the helper
    let helper = MyUpgradeHelper {
        network: used_network,
        target_version: target_version,
    };

    // Check the target version
    helper.check_target_version();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_target_version_local_node_correct_version() {
        let helper = MyUpgradeHelper { 
            network: Network::LocalNode, 
            target_version: "v14.0.0".to_string() ,
        };
        helper.check_target_version();
        // Add assertions to validate the behavior for LOCAL_NODE
    }

    #[test]
    fn test_check_target_version_testnet() {
        let helper = MyUpgradeHelper {
            network: Network::LocalNode,
            target_version: "v14,0.0".to_string()
        };
        helper.check_target_version();
        // Add assertions to validate the behavior for TESTNET
    }

    #[test]
    fn test_check_target_version_mainnet() {
        let helper = MyUpgradeHelper {
            network: Network::Mainnet,
            target_version: "v14,0.0".to_string(),
        };
        helper.check_target_version();
        // Add assertions to validate the behavior for MAINNET
    }
}