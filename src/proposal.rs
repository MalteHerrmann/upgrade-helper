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
        .register_template_file("proposal", "templates/proposal.hbs")
        .unwrap();

    let data = json!({
        "author": "Malte Herrmann, Evmos Core Team",
        "name": proposal_name,
        "height": 0, // TODO: get height from Mintscan?
        "version": target_version,
        "prev_version": previous_version,
        "features": "- neue Features",
        "diff_link": format!("https://github.com/evmos/evmos/compare/{}..{}", 
            previous_version, 
            target_version,
        ),
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
        assert!(result.is_ok())
    }
}