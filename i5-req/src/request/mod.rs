pub mod blocking;

use crate::types::{
    i5_error::I5RequestError, i5_request::ValidatedI5Request, i5_request_url::I5RequestUrl,
};

pub async fn i5_http_post(
    valid_body: ValidatedI5Request,
    url: I5RequestUrl,
) -> Result<(), I5RequestError> {
    let body = valid_body.to_json_string()?;
    let client = reqwest::Client::new();
    client
        .post(url.to_url())
        .header("Conten-Type", "application/json")
        .body(body)
        .send()
        .await
        .map_err(I5RequestError::RequestError)?;
    Ok(())
}
