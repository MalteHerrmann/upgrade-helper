use crate::helper::UpgradeHelper;
use crate::network::Network;
use crate::release::{get_asset_string, get_release};
use handlebars::{Handlebars, RenderError};
use serde_json::json;
use std::{io, process};

/// Prepares the command to submit the proposal using the Evmos CLI.
pub async fn prepare_command(helper: &UpgradeHelper) -> Result<String, RenderError> {
    let description = match get_description_from_md(&helper.proposal_file_name) {
        Ok(description) => description,
        Err(e) => {
            println!("Error reading proposal file: {}", e);
            process::exit(1);
        }
    };

    let res = get_release(helper.target_version.as_str()).await;
    let release = match res {
        Ok(release) => release,
        Err(_) => {
            println!("Release {} does not exist yet.", helper.target_version);
            process::exit(1);
        }
    };

    let assets = match get_asset_string(&release).await {
        Some(assets) => assets,
        None => {
            println!(
                "Could not generate asset string for release {}.",
                helper.target_version
            );
            process::exit(1);
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

        let command = prepare_command(&helper)
            .await
            .expect("Failed to prepare command");

        let expected_command = format!("evmosd tx gov submit-legacy-proposal software-upgrade v14.0.0 --description \"This is a test proposal.\" --upgrade-height 1000 --from testnet-address --keyring-backend test --chain-id evmos_9000-1 --home {}/.tmp-evmosd --fees 10000000aevmos --node https://tm.evmos-testnet.lava.build:26657", helper.home.as_os_str().to_str().unwrap());
        assert_eq!(command, expected_command, "command does not match");
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
