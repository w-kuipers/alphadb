// Copyright (C) 2024 Wibo Kuipers
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use crate::config::connection::get_active_connection;
use aes_gcm::aead::{Aead, KeyInit, OsRng};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use base64::engine::{general_purpose, Engine};
use colored::Colorize;
use rand_core::RngCore;

/// Print function title and current
/// database connection to the commandline
///
/// - title: Title that will be displayed
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
/// Print an error message to the
/// command line and end the process
///
/// - error_string: The error message
pub fn error(error_string: String) -> ! {
    // Some error messages are still wrapped in their definition
    let start = error_string.find("{").map(|pos| pos + 1).unwrap_or(0);
    let end = error_string.rfind("}").unwrap_or(error_string.len());

    let clean_error = &error_string[start..end].trim();
    panic!("{clean_error}");
}

#[cfg(not(debug_assertions))]
/// Debug version for the above
/// error function. Panics.
///
/// - error_string: The error message
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
    let secret_decoded = secret_decoded.unwrap();

    let cipher = Aes256Gcm::new_from_slice(&secret_decoded);

    // Generate a 12-byte nonce
    if cipher.is_ok() {
        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher.unwrap().encrypt(nonce, password.as_bytes());

        if ciphertext.is_err() {
            error("An unexpected error occured".to_string());
        }
        let ciphertext_encoded = general_purpose::STANDARD.encode(ciphertext.unwrap());
        let nonce_encoded = general_purpose::STANDARD.encode(nonce_bytes);

        return format!("{}.{}", ciphertext_encoded, nonce_encoded);
    } else {
        error("An unexpected error occured".to_string());
    }
}

pub fn decrypt_password(password: String, secret: String) -> String {
    let secret_decoded = general_purpose::STANDARD.decode(secret);
    if secret_decoded.is_err() {
        error("Error decoding use secret".to_string());
    }
    let secret_decoded = secret_decoded.unwrap();

    let cipher = Aes256Gcm::new_from_slice(&secret_decoded);
    if cipher.is_err() {
        error("Error decoding use secret".to_string());
    }
    let cipher = cipher.unwrap();

    // Split password and nonce
    let password_split = password.split(".").collect::<Vec<&str>>();
    let ciphertext = general_purpose::STANDARD.decode(password_split[0]);
    let nonce = general_purpose::STANDARD.decode(password_split[1]);

    if nonce.is_err() || ciphertext.is_err() {
        error("Unable to decode password".to_string());
    }

    // Decrypt the password
    let decrypted_bytes = cipher.decrypt(
        Nonce::from_slice(&nonce.unwrap()),
        ciphertext.unwrap().as_slice(),
    );
    if decrypted_bytes.is_err() {
        error("Unable to decrypt password".to_string());
    }

    // Convert it back to a string
    let decrypted_password = String::from_utf8(decrypted_bytes.unwrap());
    if decrypted_password.is_err() {
        error("Unable to decrypt password".to_string());
    }

    return decrypted_password.unwrap();
}
