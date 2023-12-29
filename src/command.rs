use crate::helper::UpgradeHelper;
use crate::network::Network;
use crate::release::{get_asset_string, get_release};
use handlebars::{Handlebars, RenderError};
use serde_json::json;
use std::io;

/// Prepares the command to submit the proposal using the Evmos CLI.
pub async fn prepare_command(helper: &UpgradeHelper) -> Result<String, RenderError> {
    let description = match get_description_from_md(&helper.proposal_file_name) {
        Ok(d) => d,
        Err(e) => {
            println!(
                "failed to read proposal file '{}': {}\n\n!!! ATTENTION !!!\nMake sure to generate the file using the corresponding CLI command first.\n",
                &helper.proposal_file_name, e
            );
            return Err(RenderError::from(e));
        }
    };

    let res = get_release(helper.target_version.as_str()).await;
    let release = match res {
        Ok(release) => release,
        Err(_) => {
            println!("Release {} does not exist yet.", helper.target_version);
            return Err(RenderError::new("release does not exist yet"));
        }
    };

    let assets = match get_asset_string(&release).await {
        Some(assets) => assets,
        None => {
            return Err(RenderError::new(format!(
                "could not generate asset string for release {}",
                helper.target_version,
            )));
        }
    };

    // TODO: get fees from network conditions?
    let fees = "10000000000aevmos";
    let key = get_key(helper.network);
    let tm_rpc = get_rpc_url(helper.network);

    let data = json!({
        "assets": assets,
        "chain_id": helper.chain_id,
        "description": description,
        "fees": fees,
        "height": helper.upgrade_height,
        "home": helper.home,
        "key": key,
        "title": helper.proposal_name,
        "tm_rpc": tm_rpc,
        "version": helper.target_version,
    });

    // TODO: print data JSON to file and then read as configuration
    println!("{}", data);

    let mut handlebars = Handlebars::new();
    handlebars.set_strict_mode(true);

    handlebars
        .register_template_file("command", "src/templates/command.hbs")
        .expect("Failed to register template file");

    handlebars.render("command", &data)
}

/// Returns the description string from the given Markdown file.
fn get_description_from_md(filename: &str) -> io::Result<String> {
    std::fs::read_to_string(filename)
}

/// Returns the key used for signing based on the network.
fn get_key(network: Network) -> String {
    match network {
        Network::Mainnet => "mainnet-address".to_string(),
        Network::Testnet => "testnet-address".to_string(),
        Network::LocalNode => "dev0".to_string(),
    }
}

/// Returns the RPC URL based on the network.
fn get_rpc_url(network: Network) -> String {
    match network {
        Network::Mainnet => "https://tm.evmos.lava.build:26657".to_string(),
        Network::Testnet => "https://tm.evmos-testnet.lava.build:26657".to_string(),
        Network::LocalNode => "http://localhost:26657".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::network::Network;
    use chrono::Utc;

    #[tokio::test]
    async fn test_prepare_command() {
        let helper = UpgradeHelper::new(Network::Testnet, "v13.0.0", "v14.0.0", Utc::now()).await;

        // Write description to file
        let description = "This is a test proposal.";
        std::fs::write(&helper.proposal_file_name, description)
            .expect("Unable to write proposal to file");

        match prepare_command(&helper).await {
            Ok(command) => {
                // Remove description file
                std::fs::remove_file(&helper.proposal_file_name)
                    .expect("failed to remove description file after test");

                let mut expected_command =
                    "evmosd tx gov submit-legacy-proposal software-upgrade v14.0.0 \\\n".to_owned();
                expected_command.push_str("--title \"Evmos Testnet v14.0.0 Upgrade\" \\\n");
                expected_command
                    .push_str(format!("--upgrade-height {} \\\n", helper.upgrade_height).as_str());
                expected_command.push_str("--description \"This is a test proposal.\" \\\n");
                expected_command.push_str("--from testnet-address \\\n");
                expected_command.push_str("--fees 10000000aevmos \\\n");
                expected_command.push_str("--chain-id evmos_9000-4 \\\n");
                expected_command.push_str(
                    format!(
                        "--home {} \\\n",
                        helper
                            .home
                            .as_os_str()
                            .to_str()
                            .expect("failed to get home directory as str")
                    )
                    .as_str(),
                );
                expected_command.push_str("--node https://tm.evmos-testnet.lava.build:26657 \\\n");
                expected_command.push_str("--upgrade-info \\\n");
                expected_command.push_str("-b sync");
                assert_eq!(
                    command, expected_command,
                    "expected different proposal command"
                );
            }
            Err(e) => {
                // Remove description file
                std::fs::remove_file(&helper.proposal_file_name)
                    .expect("failed to remove description file after test");
                assert!(false, "unexpected error while preparing command: {}", e);
            }
        }
    }

    #[test]
    fn test_get_description_from_md() {
        let description = get_description_from_md("src/templates/command.hbs");
        assert_eq!(
            description.is_ok(),
            true,
            "description should be ok, but is not"
        );
    }

    #[test]
    fn test_get_description_from_md_invalid_file() {
        let description = get_description_from_md("src/templates/command.hbs.invalid");
        assert_eq!(
            description.is_err(),
            true,
            "description should be err, but is not"
        );
    }

    #[test]
    fn test_get_key() {
        let key = get_key(Network::Mainnet);
        assert_eq!(key, "mainnet-address", "key does not match");

        let key = get_key(Network::Testnet);
        assert_eq!(key, "testnet-address", "key does not match");

        let key = get_key(Network::LocalNode);
        assert_eq!(key, "dev0", "key does not match");
    }

    #[test]
    fn test_get_rpc_url() {
        let rpc = get_rpc_url(Network::Mainnet);
        assert_eq!(
            rpc, "https://tm.evmos.lava.build:26657",
            "rpc does not match"
        );

        let rpc = get_rpc_url(Network::Testnet);
        assert_eq!(
            rpc, "https://tm.evmos-testnet.lava.build:26657",
            "rpc does not match"
        );

        let rpc = get_rpc_url(Network::LocalNode);
        assert_eq!(rpc, "http://localhost:26657", "rpc does not match");
    }
}
