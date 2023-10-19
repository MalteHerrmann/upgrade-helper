use std::fmt;

// Enum to represent different network options
#[derive(Debug)]
pub enum Network {
    LocalNode,
    Testnet,
    Mainnet,
}

impl fmt::Display for Network {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Network::LocalNode => write!(f, "Local Node"),
            Network::Testnet => write!(f, "Testnet"),
            Network::Mainnet => write!(f, "Mainnet"),
        }
    }
}
