use crate::error::Error;
use base64::Engine;
use http::uri::Uri;
use nostr_types::{EventKind, PreEvent, Signer, Tag, Unixtime};

pub fn auth(uri: &Uri, payload: &str) -> Result<String, Error> {
    use bitcoin_hashes::{sha256, Hash};
    let hash = sha256::Hash::hash(payload.as_bytes());
    let hashref = <sha256::Hash as AsRef<[u8; 32]>>::as_ref(&hash);
    let payload_hash = hex::encode(hashref);

    let signer = crate::signer::load_signer()?;

    let pre_event = PreEvent {
        pubkey: signer.public_key(),
        created_at: Unixtime::now().unwrap(),
        kind: EventKind::HttpAuth,
        tags: vec![
            Tag::new(&["u", &format!("{}", uri)]),
            Tag::new(&["payload", &payload_hash]),
        ],
        content: "".to_string(),
    };
    let event = signer.sign_event(pre_event)?;
    let event_string = serde_json::to_string(&event)?;
    let event_base64 = base64::engine::general_purpose::STANDARD.encode(&event_string);
    Ok(format!("Nostr {}", event_base64))
}
