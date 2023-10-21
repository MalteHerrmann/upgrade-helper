use crate::network::Network;

pub struct UpgradeHelper {
    pub network: Network,
    pub previous_version: String,
    pub target_version: String,
    pub proposal_name: String,
}

impl UpgradeHelper {
    /// Creates a new instance of the upgrade helper.
    pub fn new(
        network: Network,
        previous_version: &str,
        target_version: &str,
    ) -> UpgradeHelper {
        let proposal_name = format!("Evmos {} {} Upgrade", network, target_version);

        UpgradeHelper {
            network,
            previous_version: previous_version.to_string(),
            target_version: target_version.to_string(),
            proposal_name,
        }
    }
}
