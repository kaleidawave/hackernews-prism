use isahc::prelude::*;

#[derive(Debug)]
pub enum GetError {
    Isahc(isahc::Error),
    SerdeJson(serde_json::Error),
}

impl From<isahc::Error> for GetError {
    fn from(error: isahc::Error) -> Self {
        GetError::Isahc(error)
    }
}

impl From<serde_json::Error> for GetError {
    fn from(error: serde_json::Error) -> Self {
        GetError::SerdeJson(error)
    }
}

pub fn make_json_get_request<T: serde::de::DeserializeOwned>(url: &str) -> Result<T, GetError> {
    let mut response = isahc::get(url)?;
    return Ok(response.json::<T>()?);
}