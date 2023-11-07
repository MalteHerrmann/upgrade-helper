use std::process;
use handlebars::{Handlebars, RenderError};
use octocrab::models::repos::Release;
use serde_json::json;
use crate::helper::UpgradeHelper;
use crate::release::get_release;

/// Prepares the command to submit the proposal using the Evmos CLI.
pub async fn prepare_command(helper: &UpgradeHelper) -> Result<String, RenderError> {
    let mut handlebars = Handlebars::new();
    handlebars.set_strict_mode(true);

    handlebars
        .register_template_file("command", "src/templates/command.hbs")
        .unwrap();

    let res = get_release(helper.target_version.as_str()).await;
    let release = match res {
        Ok(release) => release,
        Err(_) => {
            println!("Release {} does not exist yet.", helper.target_version);
            process::exit(1);
        }
    };

    let assets = "TODO: assets";
    let description = "TODO: description";
    let fees = "TODO: fees";
    let key = "TODO: key";
    let tm_rpc = "TODO: tm_rpc";

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

    handlebars.render("command", &data)
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use crate::network::Network;
    use super::*;

    #[tokio::test]
    async fn test_prepare_command() {
        let helper= UpgradeHelper::new(
            Network::LocalNode,
            "v13.0.0",
            "v14.0.0",
            Utc::now(),
        ).await;

        let command = prepare_command(&helper)
            .await.expect("Failed to prepare command");

        let expected_command = format!("evmosd tx gov submit-proposal software-upgrade Test Proposal v14.0.0 --description \"This is a test proposal.\" --upgrade-height 1000 --upgrade-time 1000 --voting-period 1000 --from evmos --keyring-backend test --chain-id evmos_9000-1 --home {}/.tmp-evmosd --fees 10000000aevmos --node tcp://localhost:26657 --yes", helper.home.as_os_str().to_str().unwrap());
        assert_eq!(command, expected_command, "command does not match");
    }
}