use crate::http::get;
use octocrab::{models::repos::{Asset, Release}, Result};
use std::collections::HashMap;

/// Sends a HTTP request to the GitHub release page and returns the response.
pub async fn get_release(version: &str) -> Result<Release> {
    let octocrab = octocrab::instance();

    octocrab
        .repos("evmos", "evmos")
        .releases()
        .get_by_tag(version)
        .await
}

/// Checks if the release for the target version already exists by
/// sending a HTTP request to the GitHub release page.
pub async fn check_release_exists(version: &str) -> bool {
    match get_release(version).await {
        Ok(_) => true,
        _ => false,
    }
}

/// Returns the asset string for the release assets.
/// The asset string is used in the Evmos CLI command.
pub async fn get_asset_string(release: &Release) -> String {
    let mut assets = String::new();

    let checksums = get_checksum_map(release.assets.clone())
        .await;

    match checksums {
        Some(checksums) => {
            assets.push_str(&format!("{:?} ", checksums));
            println!("checksum: {:?}", checksums);
        }
        None => {
            println!("checksum.txt not found in release assets");
        }
    }

    for asset in &release.assets {
        assets.push_str(&format!("{} ", asset.name));
    }

    assets
}

/// Returns the checksum from the release assets.
fn get_checksum_from_assets(assets: &[octocrab::models::repos::Asset]) -> Option<&Asset> {
    // TODO: improve handling here? use getter?
    for asset in assets {
        if asset.name == "checksums.txt" {
            return Some(asset)
        }
    }

    None
}

/// Downloads the checksum file from the release assets and returns the built checksum string.
async fn get_checksum_map(assets: Vec<Asset>) -> Option<HashMap<String, String>> {
    let checksum = get_checksum_from_assets(assets.as_slice())?;

    match get(checksum.browser_download_url.clone()).await {
        Ok(response) => {
            let body = response.text().await.unwrap();
            let mut checksums = HashMap::new();

            for line in body.lines() {
                let line = line.trim();
                let parts: Vec<&str> = line.split_whitespace().collect();

                if parts.len() != 2 {
                    println!("Invalid checksum line: {}", line);
                    continue
                }

                // NOTE: Windows links are not supported in the submit-legacy-proposal command
                if parts[1].contains("Windows") {
                    continue
                }

                checksums.insert(parts[1].to_string(), parts[0].to_string());
            }

            return Some(checksums)
        }
        Err(_) => return None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_release_pass() {
        let release = get_release("v14.0.0").await.unwrap();
        assert_eq!(release.tag_name, "v14.0.0");
    }

    #[tokio::test]
    async fn test_get_release_fail() {
        let res = get_release("invalidj.xjaf/ie").await;
        assert_eq!(res.is_err(), true);
    }

    #[tokio::test]
    async fn test_check_release_exists_pass() {
        assert_eq!(check_release_exists("v14.0.0").await, true);
    }

    #[tokio::test]
    async fn test_check_release_exists_fail() {
        assert_eq!(check_release_exists("v14.0.8").await, false);
    }

    #[tokio::test]
    async fn test_get_checksum_map_pass() {
        let release = get_release("v14.0.0").await.unwrap();
        let checksums = get_checksum_map(release.assets.clone()).await.unwrap();

        assert!(checksums.contains_key("evmos_14.0.0_Linux_amd64.tar.gz"));
        assert!(checksums.contains_key("evmos_14.0.0_Linux_arm64.tar.gz"));
        assert!(checksums.contains_key("evmos_14.0.0_Darwin_amd64.tar.gz"));
        assert!(checksums.contains_key("evmos_14.0.0_Darwin_arm64.tar.gz"));
    }
}
