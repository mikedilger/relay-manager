use crate::error::{Error, InnerError};
use http::uri::{Scheme, Uri};
use reqwest::blocking::Client;
use reqwest::redirect::Policy;
use serde_json::Value;
use std::time::Duration;

pub fn post(uri: &Uri, body: String) -> Result<Value, Error> {
    eprintln!("CMD: {}", body);

    let host = {
        let authority = uri.authority().expect("Has no hostname").as_str();
        authority
            .find('@')
            .map(|idx| authority.split_at(idx + 1).1)
            .unwrap_or_else(|| authority)
            .to_owned()
    };
    let mut parts = uri.clone().into_parts();
    if host.is_empty() {
        panic!("URL has empty hostname");
    }
    parts.scheme = match parts
        .scheme
        .ok_or::<Error>(InnerError::MissingScheme.into())?
        .as_str()
    {
        "wss" => Some(Scheme::HTTPS),
        "ws" => Some(Scheme::HTTP),
        "https" => Some(Scheme::HTTPS),
        "http" => Some(Scheme::HTTP),
        _ => panic!("We don't support that scheme."),
    };
    let uri = Uri::from_parts(parts)?;

    let auth = crate::auth::auth(&uri, &body)?;

    let client = Client::builder()
        .redirect(Policy::none())
        .connect_timeout(Some(Duration::from_secs(20)))
        .timeout(Some(Duration::from_secs(20)))
        .connection_verbose(true)
        .build()?;
    let response = client
        .post(format!("{}", uri))
        .header("Host", host)
        .header("Accept", "application/nostr+json+rpc")
        .header("Authorization", auth)
        .body(body)
        .send()?;

    let status = response.status().as_u16();
    let response_text = response.text()?;
    if status != 200 {
        return Err(InnerError::ServerError(status, response_text).into());
    }

    let value: serde_json::Value = serde_json::from_str(&response_text)
        .map_err(|e| <InnerError as Into<Error>>::into(InnerError::UnrecognizedResponse(e)))?;
    Ok(value)
}
