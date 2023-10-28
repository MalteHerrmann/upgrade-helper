use reqwest::blocking::{get as blocking_get, Response};
use url::Url;

// Queries the given URL.
pub fn get(url: Url) -> Result<Response, reqwest::Error> {
    blocking_get(url)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_pass() {
        let url = Url::parse("https://httpbin.org/get").unwrap();
        let resp = get(url).unwrap();
        assert_eq!(resp.status().is_success(), true);
    }

    #[test]
    fn test_get_fail() {
        let url = Url::parse("https://invalidurl.org/get").unwrap();
        let res = get(url);
        assert_eq!(res.is_err(), true);
    }
}