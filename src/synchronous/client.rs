use super::error::{SirixError, SirixResult};
use serde::de::DeserializeOwned;
use serde_json;
use std::io::Read;
use ureq;

#[derive(Debug)]
pub struct SirixResponse<T> {
    pub status: u16,
    pub etag: Option<String>,
    pub body: T,
}

pub fn request<T: DeserializeOwned>(
    req: ureq::Request,
    body: Option<&str>,
) -> SirixResult<SirixResponse<T>> {
    let response = match body {
        Some(data) => req.send_string(data),
        None => req.call(),
    };

    match response {
        Ok(resp) => {
            let status = resp.status();
            let etag = resp.header("etag").map(String::from);
            match serde_json::from_reader(resp.into_reader()) {
                Ok(parsed) => Ok(SirixResponse {
                    body: parsed,
                    status,
                    etag,
                }),
                Err(err) => Err(SirixError::FormatError(err)),
            }
        }
        Err(err) => Err(SirixError::ConnectionError(err)),
    }
}

pub fn request_empty(
    req: ureq::Request,
    body: Option<&str>,
) -> SirixResult<SirixResponse<()>> {
    let response = match body {
        Some(data) => req.send_string(data),
        None => req.call(),
    };

    match response {
        Ok(resp) => {
            let status = resp.status();
            let etag = resp.header("etag").map(String::from);
            Ok(SirixResponse {
                body: (),
                status,
                etag,
            })
        }
        Err(err) => Err(SirixError::ConnectionError(err)),
    }
}

pub fn request_string(
    req: ureq::Request,
    body: Option<&str>,
) -> SirixResult<SirixResponse<String>> {
    let response = match body {
        Some(data) => req.send_string(data),
        None => req.call(),
    };

    match response {
        Ok(resp) => {
            let status = resp.status();
            let etag = resp.header("etag").map(String::from);
            let mut buf: Vec<u8> = vec![];
            resp.into_reader().read_to_end(&mut buf).unwrap();
            Ok(SirixResponse {
                body: String::from_utf8_lossy(&buf).into_owned(),
                status,
                etag,
            })
        }
        Err(err) => Err(SirixError::ConnectionError(err)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::mock;

    fn agent() -> ureq::Agent {
        ureq::agent()
    }

    #[test]
    fn request_parses_json_response_body() {
        let _m = mock("GET", "/client-test-parse")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"key": "value"}"#)
            .create();

        let req = agent().get(&format!("{}/client-test-parse", mockito::server_url()));
        let result: SirixResult<SirixResponse<serde_json::Value>> = request(req, None);
        let resp = result.unwrap();
        assert_eq!(resp.status, 200);
        assert_eq!(resp.body["key"], "value");
    }

    #[test]
    fn request_captures_etag_header() {
        let _m = mock("GET", "/client-test-etag")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_header("etag", "\"abc123\"")
            .with_body(r#"{"ok": true}"#)
            .create();

        let req = agent().get(&format!("{}/client-test-etag", mockito::server_url()));
        let result: SirixResult<SirixResponse<serde_json::Value>> = request(req, None);
        let resp = result.unwrap();
        assert_eq!(resp.etag, Some("\"abc123\"".to_string()));
    }

    #[test]
    fn request_returns_none_etag_when_absent() {
        let _m = mock("GET", "/client-test-no-etag")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"true"#)
            .create();

        let req = agent().get(&format!(
            "{}/client-test-no-etag",
            mockito::server_url()
        ));
        let result: SirixResult<SirixResponse<bool>> = request(req, None);
        let resp = result.unwrap();
        assert_eq!(resp.etag, None);
    }

    #[test]
    fn request_returns_format_error_on_invalid_json() {
        let _m = mock("GET", "/client-test-bad-json")
            .with_status(200)
            .with_body("not valid json {{{")
            .create();

        let req = agent().get(&format!(
            "{}/client-test-bad-json",
            mockito::server_url()
        ));
        let result: SirixResult<SirixResponse<serde_json::Value>> = request(req, None);
        assert!(matches!(result, Err(SirixError::FormatError(_))));
    }

    #[test]
    fn request_returns_connection_error_on_http_error() {
        let _m = mock("GET", "/client-test-500")
            .with_status(500)
            .with_body("server error")
            .create();

        let req = agent().get(&format!("{}/client-test-500", mockito::server_url()));
        let result: SirixResult<SirixResponse<serde_json::Value>> = request(req, None);
        assert!(matches!(result, Err(SirixError::ConnectionError(_))));
    }

    #[test]
    fn request_sends_body_when_provided() {
        let _m = mock("POST", "/client-test-body")
            .match_body(r#"{"data":"test"}"#)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"received": true}"#)
            .create();

        let req = agent().post(&format!("{}/client-test-body", mockito::server_url()));
        let result: SirixResult<SirixResponse<serde_json::Value>> =
            request(req, Some(r#"{"data":"test"}"#));
        let resp = result.unwrap();
        assert_eq!(resp.body["received"], true);
    }

    #[test]
    fn request_string_returns_raw_body() {
        let _m = mock("GET", "/client-test-string")
            .with_status(200)
            .with_body("hello world")
            .create();

        let req = agent().get(&format!(
            "{}/client-test-string",
            mockito::server_url()
        ));
        let result = request_string(req, None);
        let resp = result.unwrap();
        assert_eq!(resp.status, 200);
        assert_eq!(resp.body, "hello world");
    }

    #[test]
    fn request_string_captures_etag() {
        let _m = mock("GET", "/client-test-string-etag")
            .with_status(200)
            .with_header("etag", "\"v2\"")
            .with_body("body")
            .create();

        let req = agent().get(&format!(
            "{}/client-test-string-etag",
            mockito::server_url()
        ));
        let result = request_string(req, None);
        let resp = result.unwrap();
        assert_eq!(resp.etag, Some("\"v2\"".to_string()));
    }

    #[test]
    fn request_string_returns_connection_error_on_http_error() {
        let _m = mock("GET", "/client-test-string-500")
            .with_status(500)
            .with_body("error")
            .create();

        let req = agent().get(&format!(
            "{}/client-test-string-500",
            mockito::server_url()
        ));
        let result = request_string(req, None);
        assert!(matches!(result, Err(SirixError::ConnectionError(_))));
    }

    #[test]
    fn request_string_sends_body_when_provided() {
        let _m = mock("POST", "/client-test-string-body")
            .match_body("post data")
            .with_status(200)
            .with_body("ok")
            .create();

        let req = agent().post(&format!(
            "{}/client-test-string-body",
            mockito::server_url()
        ));
        let result = request_string(req, Some("post data"));
        let resp = result.unwrap();
        assert_eq!(resp.body, "ok");
    }
}
