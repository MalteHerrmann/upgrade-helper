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
}

impl UpgradeHelper for MyUpgradeHelper {
    const DEFAULT_HOME: &'static str = "/path/to/default/home";

    fn check_target_version(&self) {
        match self.network {
            Network::LocalNode => {
                println!("Checking target version for LOCAL_NODE");
                // Check target version logic for LOCAL_NODE
            },
            Network::Testnet => {
                println!("Checking target version for TESTNET");
                // Check target version logic for TESTNET
            },
            Network::Mainnet => {
                println!("Checking target version for MAINNET");
                // Check target version logic for MAINNET
            },
        }
    }
}

fn main() {
    let network_options = vec![
        "Local Node",
        "Testnet",
        "Mainnet",
    ];

    // Initialize the network variable
    let used_network: Network;

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

    // Create an instance of the helper
    let helper = MyUpgradeHelper {
        network: used_network,
    };

    // Check the target version
    helper.check_target_version();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_target_version_local_node() {
        let network = Network::LocalNode;
        let helper = MyUpgradeHelper {network};
        helper.check_target_version();
        // Add assertions to validate the behavior for LOCAL_NODE
    }

    #[test]
    fn test_check_target_version_testnet() {
        let network = Network::Testnet;
        let helper = MyUpgradeHelper {network};
        helper.check_target_version();
        // Add assertions to validate the behavior for TESTNET
    }

    #[test]
    fn test_check_target_version_mainnet() {
        let network = Network::Mainnet;
        let helper = MyUpgradeHelper {network};
        helper.check_target_version();
        // Add assertions to validate the behavior for MAINNET
    }
}