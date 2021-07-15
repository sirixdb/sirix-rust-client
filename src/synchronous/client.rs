use super::error::{SirixError, SirixResult};
use serde::de::DeserializeOwned;
use serde_json;
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

pub fn request_string(req: ureq::Request, body: Option<&str>) -> SirixResult<SirixResponse<String>> {
    let response = match body {
        Some(data) => req.send_string(data),
        None => req.call(),
    };

    match response {
        Ok(resp) => {
            let status = resp.status();
            let etag = resp.header("etag").map(String::from);
            Ok(SirixResponse {
                body: resp.into_string().unwrap(),
                status,
                etag,
            })
        }
        Err(err) => Err(SirixError::ConnectionError(err)),
    }
}
