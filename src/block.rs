extern crate reqwest;
use crate::network::Network;
use chrono::{DateTime, Utc};

/// Represents a block from the Evmos network.
#[derive(Debug)]
pub struct Block {
    height: u64,
    time: DateTime<Utc>,
}

/// Gets the estimated block height for the given upgrade time.
pub fn get_estimated_height(upgrade_time: DateTime<Utc>) -> u64 {
    let block = get_latest_block(Network::Mainnet);
    println!("Block: {:?}", block);

    block.height
}

/// Gets the latest block from the Evmos network.
fn get_latest_block(network: Network) -> Block {
    let url: &str;
    url = match network {
        Network::LocalNode => "http://localhost:1317/cosmos/base/tendermint/v1beta1/blocks/latest",
        Network::Mainnet => "https://rest.evmos.lava.build/cosmos/base/tendermint/v1beta1/blocks/latest",
        Network::Testnet => "https://rest.evmos-testnet.lava.build/cosmos/base/tendermint/v1beta1/blocks/latest",
    };
    let response = reqwest::blocking::get(url).unwrap();
    println!("Status: {}", response.status());
    let block = process_block_body(response.text().unwrap());
    println!("block: {:?}", block);
    block
}

/// Processes the block body.
fn process_block_body(body: String) -> Block {
    // build regex to find the block height
    let re = regex::Regex::new(
        r#""height: "(\d+)","time":"([0-9\-T\:]+)""#,
    ).unwrap();
    let captures = re.captures(&body).unwrap();

    let captured_height = captures.get(1).map_or("", |m| m.as_str());
    let captured_time = captures.get(2).map_or("", |m| m.as_str());
    let parsed_height = captured_height.parse::<u64>();
    let height: u64;
    match parsed_height {
        Ok(h) => height = h,
        Err(_) => panic!("Could not parse block height"),
    }


    println!("time: {}", captured_time);
    let fmt = "%Y-%m-%dT%H:%M:%S%.fZ";
    let time: DateTime<Utc>;
    let time_res = chrono::DateTime::parse_from_str(
        captured_time, fmt,
    );
    match time_res {
        Ok(t) => time = t.with_timezone(&Utc),
        Err(_) => panic!("Could not parse block time"),
    }

    Block {
        height,
        time,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::network::Network;

    #[test]
    fn test_get_latest_block_mainnet() {
        let block = get_latest_block(Network::Mainnet);
        assert!(block.height > 0);
    }

    #[test]
    fn test_get_latest_block_testnet() {
        let block = get_latest_block(Network::Testnet);
        assert!(block.height > 0);
    }
}