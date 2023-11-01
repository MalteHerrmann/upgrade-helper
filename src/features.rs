use crate::release::get_release;

// Queries the changelog from the release
async fn get_changelog(version: &str) -> String {
    let res = get_release(version).await;
    match res {
        Ok(release) => {
            let body = release.body;
            match body {
                Some(body) => {
                    println!("body: {}", body);
                    body
                },
                None => String::from(""),
            }
        }
        Err(_) => String::from(""),
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[tokio::test]
    async fn test_get_changelog_pass() {
        let changelog = get_changelog("v14.0.0").await;
        assert_ne!(changelog.as_str(), "", "expected a changelog body to be returned");
    }

    #[tokio::test]
    async fn test_get_changelog_fail() {
        let changelog = get_changelog("v14.0.8").await;
        assert_eq!(changelog.as_str(), "", "expected no changelog body to be returned");
    }
}
