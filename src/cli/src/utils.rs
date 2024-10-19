use crate::config::connection::get_active_connection;
use aes_gcm::aead::{Aead, KeyInit, OsRng};
use aes_gcm::{Aes256Gcm, Nonce, Key};
use base64::engine::{general_purpose, Engine};
use colored::Colorize;
use rand_core::RngCore;

pub fn title(title: &str) {
    if let Some(conn) = get_active_connection() {
        println!(
            "{} {} {} {}:{}",
            "Connected to database".cyan(),
            conn.database,
            "on".cyan(),
            conn.host,
            conn.port
        );
    }

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
    use std::process;
    let start = error_string.find("{").map(|pos| pos + 1).unwrap_or(0);
    let end = error_string.rfind("}").unwrap_or(error_string.len());

    let clean_error = &error_string[start..end].trim();

    eprintln!("\n{}\n", clean_error.red());
    process::exit(1);
}

pub fn encrypt_password(password: &str, secret: String) -> String {
    let secret_decoded = general_purpose::STANDARD.decode(secret);
    if secret_decoded.is_err() {
        error("Error decoding use secret".to_string());
    }
    let mut secret_decoded = secret_decoded.unwrap();

    OsRng.fill_bytes(&mut secret_decoded);
    let cipher = Aes256Gcm::new_from_slice(&secret_decoded);

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
}

pub fn decrypt_password(
    password: String,
    secret: String,
) -> String {
    let secret_decoded = general_purpose::STANDARD.decode(secret);
    if secret_decoded.is_err() {
        error("Error decoding use secret".to_string());
    }
    let secret_decoded = secret_decoded.unwrap();

    // let key = Key::from_slice(&secret_decoded);
    // let cipher = Aes256Gcm::new(key);
    let cipher = Aes256Gcm::new_from_slice(&secret_decoded);
    if cipher.is_err() {
        error("Error decoding use secret".to_string());
    }
    let cipher = cipher.unwrap();

    // Split password and nonce
    let password_split = password.split(".").collect::<Vec<&str>>();
    let nonce = general_purpose::STANDARD.decode(password_split[0]);
    let ciphertext = general_purpose::STANDARD.decode(password_split[1]);

    if nonce.is_err() || ciphertext.is_err() {
        error("Unable to decode password".to_string());
    }

    // Decrypt the password
    let decrypted_bytes = cipher.decrypt(Nonce::from_slice(&nonce.unwrap()), ciphertext.unwrap().as_ref());
    if decrypted_bytes.is_err() {
        error("Unable to decrypt password".to_string());
    }

    let decrypted_password = String::from_utf8(decrypted_bytes.unwrap());
    if decrypted_password.is_err() {
        error("Unable to decrypt password".to_string());
    }


    return decrypted_password.unwrap();
}
