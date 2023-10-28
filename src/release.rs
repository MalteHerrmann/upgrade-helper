use crate::http::get;
use reqwest::{Error, StatusCode};
use url::Url;

/// Checks if the release for the target version already exists by
/// sending a HTTP request to the GitHub release page.
pub fn check_release_exists(version: &str) -> Result<bool, Error> {
    let release_url = Url::parse("https://github.com/evmos/evmos/releases/tag/")
        .expect("the release URL should be valid")
        .join(version)
        .expect("joining the release version should be valid");

    let resp = get(release_url)?;
    match resp.status() {
        StatusCode::NOT_FOUND => Ok(false),
        _ => Ok(true),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_release_exists_pass() {
        assert_eq!(check_release_exists("v14.0.0").unwrap(), true);
    }

    #[test]
    fn test_check_release_exists_fail() {
        assert_eq!(check_release_exists("v14.0.8").unwrap(), false);
    }
}
