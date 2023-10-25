extern crate reqwest;
use crate::network::Network;
use chrono::{DateTime, TimeZone, Utc};
use regex::Captures;

/// Represents a block from the Evmos network.
#[derive(Debug)]
pub struct Block {
    height: u64,
    time: DateTime<Utc>,
}

/// Gets the estimated block height for the given upgrade time.
pub fn get_estimated_height(network: Network, upgrade_time: DateTime<Utc>) -> u64 {
    let block = get_latest_block(network);
    println!("Block: {:?}", block);

    block.height
}

/// Gets the latest block from the Evmos network.
fn get_latest_block(network: Network) -> Block {
    let url: &str;
    url = match network {
        Network::LocalNode => "http://localhost:1317/cosmos/base/tendermint/v1beta1/blocks/latest",
        Network::Mainnet => {
            "https://rest.evmos.lava.build/cosmos/base/tendermint/v1beta1/blocks/latest"
        }
        Network::Testnet => {
            "https://rest.evmos-testnet.lava.build/cosmos/base/tendermint/v1beta1/blocks/latest"
        }
    };
    let response =
        reqwest::blocking::get(url).expect("the latest block should be successfully queried");

    process_block_body(response.text().unwrap())
}

/// Processes the block body.
fn process_block_body(body: String) -> Block {
    // build regex to find the block height
    let re = regex::Regex::new(r#"height\":\"(\d+)\",\"time\":\"([T0-9\-\:]+)"#).unwrap();

    let captures: Captures;
    let captures_res = re.captures(&body);
    match captures_res {
        None => panic!("failed to parse block response body"),
        Some(c) => captures = c,
    }

    // Extract the block height
    let captured_height = captures.get(1).map_or("", |m| m.as_str());
    let parsed_height = captured_height.parse::<u64>();
    let height: u64;
    match parsed_height {
        Ok(h) => height = h,
        Err(_) => panic!("Could not parse block height"),
    }

    // Parse the block time
    let captured_time = captures.get(2).map_or("", |m| m.as_str());
    let time_format = "%Y-%m-%dT%H:%M:%S";
    let time_res = chrono::NaiveDateTime::parse_from_str(captured_time, time_format);
    let time: DateTime<Utc>;
    match time_res {
        Ok(t) => time = Utc.from_utc_datetime(&t),
        Err(e) => panic!("Could not parse block time: {}", e),
    }

    Block { height, time }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::network::Network;
    use chrono::TimeZone;

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

    #[test]
    fn test_process_block_body_pass() {
        let body = r#"{"block_id":{"hash":"CDHpDYu4tRibegIDTHust45sWB6ebnNE0Wq4sMpbSP8=","part_set_header":{"total":1,"hash":"bLAKlbU5Y0rqC1p07Xuhxm355sa+wPxwD9roDtnIzqA="}},"block":{"header":{"version":{"block":"11","app":"0"},"chain_id":"evmos_9001-2","height":"16699401","time":"2023-10-25T10:09:34.440526177Z","last_block_id""#;
        let block = process_block_body(body.to_string());

        assert_eq!(block.height, 16699401, "expected a different block height");
        assert_eq!(
            block.time,
            Utc.with_ymd_and_hms(2023, 10, 25, 10, 09, 34).unwrap(),
            "expected a different block time",
        );
    }
}
