use crate::{helper::UpgradeHelper, inputs::get_time_string, network::Network};
use handlebars::{Handlebars, RenderError};
use num_format::ToFormattedString;
use serde_json::json;

/// Prepares the proposal text by filling in the necessary information
/// to the proposal template.
pub fn prepare_proposal(helper: &UpgradeHelper) -> Result<String, RenderError> {
    let mut handlebars = Handlebars::new();
    handlebars.set_strict_mode(true);

    handlebars
        .register_template_file("proposal", "src/templates/proposal.hbs")
        .unwrap();

    let height_link = get_height_with_link(helper.network, helper.upgrade_height);

    let data = json!({
        "author": "Malte Herrmann, Evmos Core Team",
        "diff_link": format!("https://github.com/evmos/evmos/compare/{}..{}",
            helper.previous_version,
            helper.target_version,
        ),
        "estimated_time": get_time_string(helper.upgrade_time),
        "features": "- neue Features",
        "height": height_link,
        "name": helper.proposal_name,
        "network": format!("{}", helper.network), // TODO: implement serialize trait here?
        "previous_version": get_release_md_link(helper.previous_version.as_str()),
        "version": get_release_md_link(helper.target_version.as_str()),
        "voting_time": helper.voting_period.num_hours(),
    });

    handlebars.render("proposal", &data)
}

/// Writes the proposal contents to a file.
pub fn write_proposal_to_file(
    proposal: &str,
    network: Network,
    target_version: &str,
) -> Result<(), std::io::Error> {
    let proposal_file_name = format!("proposal-{}-{}.md", network, target_version);
    std::fs::write(proposal_file_name, proposal)
}

/// Returns the appropriate Markdown link to the block on Mintscan for the given network and height.
fn get_height_with_link(network: Network, height: u64) -> String {
    let height_with_commas = height.to_formatted_string(&num_format::Locale::en);
    match network {
        Network::LocalNode => format!("[{}](https://www.mintscan.io/evmos/blocks/{})", height_with_commas, height),
        Network::Mainnet => format!("[{}](https://www.mintscan.io/evmos/blocks/{})", height_with_commas, height),
        Network::Testnet => format!("[{}](https://testnet.mintscan.io/evmos-testnet/blocks/{})", height_with_commas, height),
    }
}

/// Returns the appropriate Markdown link to the release on GitHub for the given version.
fn get_release_md_link(version: &str) -> String {
    format!("[{0}](https://github.com/evmos/evmos/releases/tag/{0})", version)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[tokio::test]
    async fn test_prepare_proposal_pass() {
        let helper = UpgradeHelper::new(Network::Mainnet, "v0.0.1", "v0.1.0", Utc::now()).await;

        let result = prepare_proposal(&helper);
        assert!(
            result.is_ok(),
            "Error rendering proposal: {}",
            result.unwrap_err(),
        );
    }

    #[test]
    fn test_write_proposal_to_file_pass() {
        let result = write_proposal_to_file("test", Network::Mainnet, "v0.1.0");
        assert!(
            result.is_ok(),
            "Error writing proposal to file: {}",
            result.unwrap_err(),
        );

        // Check that file exists
        let proposal_file_name = format!("proposal-{}-{}.md", Network::Mainnet, "v0.1.0");
        assert!(
            std::path::Path::new(proposal_file_name.as_str()).exists(),
            "Proposal file does not exist",
        );

        // Clean up
        std::fs::remove_file(proposal_file_name).unwrap();
    }
}
