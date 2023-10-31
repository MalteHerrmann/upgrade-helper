use reqwest::{get as getReqwest, Response};
use url::Url;

// Queries the given URL.
pub async fn get(url: Url) -> reqwest::Result<Response> {
    getReqwest(url).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_pass() {
        let url = Url::parse("https://httpbin.org/get").unwrap();
        let resp = get(url).await.expect("the request should be successful");
        assert_eq!(resp.status().is_success(), true);
    }

    #[tokio::test]
    async fn test_get_fail() {
        let url = Url::parse("https://invalidurl.org/get").unwrap();
        let res = get(url).await;
        assert_eq!(res.is_err(), true);
    }
}
