use octocrab::{models::repos::Release, Result};

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
}
