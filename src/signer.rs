use crate::error::Error;
use nostr_types::{EncryptedPrivateKey, KeySigner, Signer};
use zeroize::Zeroize;

pub fn load_signer() -> Result<KeySigner, Error> {
    let mut epk_file = match dirs::config_dir() {
        Some(cd) => cd,
        None => panic!("No config_dir defined for your operating system"),
    };
    epk_file.push("relay-manager");
    epk_file.push("epk");

    let epk_bytes = match std::fs::read(&epk_file) {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!(
                "Could not find your encrypted private key in {}: {}",
                epk_file.display(),
                e
            );
            let epk = rpassword::prompt_password("ncryptsec1: ").unwrap();
            epk.as_bytes().to_vec()
        }
    };
    let epk_string = String::from_utf8(epk_bytes)?;
    let epk = EncryptedPrivateKey(epk_string);

    let mut password = rpassword::prompt_password("Password: ").unwrap();
    let mut signer = KeySigner::from_encrypted_private_key(epk, &password)?;
    signer.unlock(&password)?;
    password.zeroize();
    Ok(signer)
}
