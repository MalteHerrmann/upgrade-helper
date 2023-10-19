extern crate reqwest;

/// Checks if the release for the target version already exists by 
/// sending a HTTP request to the GitHub release page.
pub fn check_release_exists(version: String) -> Result<bool, reqwest::Error> {
    let release_url = format!("https://github.com/evmos/evmos/releases/tag/{}", version);

    let resp = reqwest::blocking::get(release_url)?;
    match resp.status() {
        reqwest::StatusCode::NOT_FOUND => {
            Ok(false)
        },
        _ => {
            Ok(true)
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_release_exists_pass() {
        assert_eq!(check_release_exists("v14.0.0".to_string()).unwrap(), true);
    }

    #[test]
    fn test_check_release_exists_fail() {
        assert_eq!(check_release_exists("v14.0.8".to_string()).unwrap(), false);
    }
}
