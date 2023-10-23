use crate::network::Network;
use chrono::Duration;

pub struct UpgradeHelper {
    pub network: Network,
    pub previous_version: String,
    pub target_version: String,
    pub proposal_name: String,
    pub voting_period: Duration,
}

impl UpgradeHelper {
    /// Creates a new instance of the upgrade helper.
    pub fn new(network: Network, previous_version: &str, target_version: &str) -> UpgradeHelper {
        let proposal_name = format!("Evmos {} {} Upgrade", network, target_version);

        let voting_period: Duration;
        match network {
            Network::LocalNode => voting_period = Duration::hours(1),
            Network::Testnet => voting_period = Duration::hours(12),
            Network::Mainnet => voting_period = Duration::hours(120),
        }

        UpgradeHelper {
            network,
            previous_version: previous_version.to_string(),
            target_version: target_version.to_string(),
            proposal_name,
            voting_period,
        }
    }
}
