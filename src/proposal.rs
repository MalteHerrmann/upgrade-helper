use handlebars::{
    Handlebars,
    RenderError,
};
use serde_json::json;
use crate::network::Network;

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
        "network": format!("{}", network),
        "previous_version": previous_version,
        "version": target_version,
        "voting_time": if matches!(network, Network::Testnet) { "12 hours" } else { "120 hours" },
    });

    handlebars.render("proposal", &data)
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
}