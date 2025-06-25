use reqwest::blocking::Response;

use crate::types::{
    i5_error::I5RequestError, i5_request::ValidatedI5Request, i5_request_url::I5RequestUrl,
};

pub fn i5_http_post(
    valid_body: ValidatedI5Request,
    url: I5RequestUrl,
    allow_untrusted_cert: bool,
) -> Result<Response, I5RequestError> {
    let body = valid_body.to_json_string()?;
    let client = reqwest::blocking::Client::builder()
        .danger_accept_invalid_certs(allow_untrusted_cert)
        .build()
        .map_err(I5RequestError::RequestError)?;

    let res = client
        .post(url.to_url())
        .header("Conten-Type", "application/json")
        .body(body)
        .send()
        .map_err(I5RequestError::RequestError)?;
    Ok(res)
}
