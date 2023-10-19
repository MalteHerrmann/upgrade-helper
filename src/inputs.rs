use crate::network::Network;
use inquire::Select;
use std::process;

/// Prompts the user to select the network type used.
pub fn get_used_network() -> Network {
    let used_network: Network;

    let network_options = vec![
        "Local Node",
        "Testnet",
        "Mainnet",
    ];

    // Prompt the user to select the network
    let chosen_network = Select::new(
        "Select network", network_options,
    ).prompt();

    match chosen_network {
        Ok(choice) => {
            match choice {
                "Local Node" => used_network = Network::LocalNode,
                "Testnet" => used_network = Network::Testnet,
                "Mainnet" => used_network = Network::Mainnet,
                &_ => {
                    println!("Invalid network selected: {:?}", choice);
                    process::exit(1); // TODO: return error here instead of exiting in here
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

pub fn get_target_version() -> String {
    let target_version: String;
    // Prompt the user to input the desired target version
    let result = inquire::Text::new("Target version to upgrade to:")
        .prompt();
    match result {
        Ok(version) => {
            target_version = version;
        }
        Err(e) => { 
            println!("Error selecting target version: {}", e); 
            process::exit(1);
        }
    }

    target_version
}