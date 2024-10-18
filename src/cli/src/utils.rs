use crate::config::setup::config_read;
use aes_gcm::aead::{Aead, KeyInit, OsRng};
use aes_gcm::{Aes256Gcm, Nonce};
use base64::engine::{general_purpose, Engine};
use colored::Colorize;
use rand_core::RngCore;
use std::process;

pub fn title(title: &str) {
    println!(
        "{} {} {} {}",
        "Connected to database".cyan(),
        "PLACEHOLDER",
        "on".cyan(),
        "PLACEHOLDER"
    );
    println!("\n{} {} {}\n", "-----".green(), title, "-----".green());
}

#[cfg(debug_assertions)]
pub fn error(error_string: String) -> ! {
    // Some error messages are still wrapped in their definition
    let start = error_string.find("{").map(|pos| pos + 1).unwrap_or(0);
    let end = error_string.rfind("}").unwrap_or(error_string.len());

    let clean_error = &error_string[start..end].trim();
    panic!("{clean_error}");
}

#[cfg(not(debug_assertions))]
pub fn error(error_string: String) -> ! {
    // Some error messages are still wrapped in their definition
    let start = error_string.find("{").map(|pos| pos + 1).unwrap_or(0);
    let end = error_string.rfind("}").unwrap_or(error_string.len());

    let clean_error = &error_string[start..end].trim();

    eprintln!("\n{}\n", clean_error.red());
    process::exit(1);
}

pub fn encrypt_password(password: &str) -> String {
    if let Some(config) = config_read() {
        let mut secret = [0u8; 32];
        OsRng.fill_bytes(&mut secret);
        let cipher = Aes256Gcm::new_from_slice(&secret);

        // Generate a 12-byte nonce
        if cipher.is_ok() {
            let mut nonce_bytes = [0u8; 12];
            OsRng.fill_bytes(&mut nonce_bytes);
            let nonce = Nonce::from_slice(&nonce_bytes);

            let ciphertext = cipher.unwrap().encrypt(nonce, password.as_bytes());

            // Ok((ciphertext, nonce_bytes.to_vec()))
            if ciphertext.is_err() {
                error("An unexpected error occured".to_string());
            }
            let ciphertext_encoded = general_purpose::STANDARD.encode(ciphertext.unwrap());
            let nonce_encoded = general_purpose::STANDARD.encode(nonce);

            return format!("{}.{}", ciphertext_encoded, nonce_encoded);
        } else {
            error("An unexpected error occured".to_string());
        }
    } else {
        error("Error occured while reading config".to_string());
    }
}

// fn decrypt_password(
//     ciphertext: &[u8],
//     nonce: &[u8],
//     key_bytes: &[u8; 32],
// ) -> Result<String, Box<dyn std::error::Error>> {
//     // Initialize AES-GCM with the same key
//     let key = Key::from_slice(key_bytes);
//     let cipher = Aes256Gcm::new(key);
//
//     // Decrypt the password
//     let decrypted_bytes = cipher.decrypt(Nonce::from_slice(nonce), ciphertext)?;
//
//     let decrypted_password = String::from_utf8(decrypted_bytes)?;
//
//     Ok(decrypted_password)
// }
