use handlebars::{
    Handlebars,
    RenderError,
};
use serde_json::json;
use crate::network::Network;

/// Prepares the proposal text by filling in the necessary information
/// to the proposal template.
pub fn prepare_proposal(
    network: Network,
    target_version: &str,
    previous_version: &str,
) -> Result<String, RenderError> {
    let proposal_name = format!("Evmos {} {} Upgrade", network, target_version);

    let mut handlebars = Handlebars::new();
    handlebars
        .set_strict_mode(true);

    handlebars
        .register_template_file("proposal", "src/templates/proposal.hbs")
        .unwrap();

    let data = json!({
        "author": "Malte Herrmann, Evmos Core Team",
        "diff_link": format!("https://github.com/evmos/evmos/compare/{}..{}", 
            previous_version, 
            target_version,
        ),
        "estimated_time": "4PM UTC, Monday, Sept. 25th, 2023",
        "features": "- neue Features",
        "height": 0, // TODO: get height from Mintscan?
        "name": proposal_name,
        "network": format!("{}", network), // TODO: implement serialize trait here?
        "previous_version": previous_version,
        "version": target_version,
        "voting_time": if matches!(network, Network::Testnet) { "12 hours" } else { "120 hours" },
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prepare_proposal_pass() {
        let result = prepare_proposal(
            Network::Mainnet,
            "v0.1.0",
            "v0.0.1",
        );
        assert!(
            result.is_ok(), 
            "Error rendering proposal: {}",
            result.unwrap_err(),
        );
    }

    #[test]
    fn test_write_proposal_to_file_pass() {
        let result = write_proposal_to_file(
            "test",
            Network::Mainnet,
            "v0.1.0",
        );
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