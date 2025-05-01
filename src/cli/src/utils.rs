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
use aes_gcm::{Aes256Gcm, Nonce};
use base64::engine::{general_purpose, Engine};
use colored::Colorize;
use rand_core::RngCore;
use std::string::FromUtf8Error;
use thiserror::Error;
use std::process;

/// Print function title and current database connection to the commandline
///
/// # Arguments
/// * `title` - Title that will be displayed
pub fn title(title: &str) {
    if let Some(conn) = get_active_connection() {
        println!(
            "{} {} {} {}:{} {}",
            "Connected to database".cyan(),
            conn.connection.database,
            "on".cyan(),
            conn.connection.host,
            conn.connection.port,
            format!("({})", conn.label).green()
        );
    }

    println!("\n{} {} {}\n", "-----".green(), title, "-----".green());
}

/// Abort the program with a message
///
/// This function prints an "Aborted" message in red and exits the program
/// with status code 0.
pub fn abort() {
    println!("{}", "\nAborted.".red());
    process::exit(0);
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! error {
    ($error_string:expr) => {
        {
            let error_string = $error_string;
            let start = error_string.find("{").map(|pos| pos + 1).unwrap_or(0);
            let end = error_string.rfind("}").unwrap_or(error_string.len());
            let clean_error = &error_string[start..end].trim();
            panic!("{}\nLocation: {}:{}:{}", clean_error, file!(), line!(), column!());
        }
    };
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! error {
    ($error_string:expr) => {
        {
            let error_string = $error_string;
            let start = error_string.find("{").map(|pos| pos + 1).unwrap_or(0);
            let end = error_string.rfind("}").unwrap_or(error_string.len());
            let clean_error = &error_string[start..end].trim();
            eprintln!("\n{}\n", clean_error.red());
            process::exit(1);
        }
    };
}

/// Encrypt a password using AES-256-GCM
///
/// This function encrypts a password using AES-256-GCM encryption with a provided secret.
/// The encrypted password is returned as a base64 encoded string containing both the
/// ciphertext and nonce.
///
/// # Arguments
/// * `password` - The password to encrypt
/// * `secret` - The base64 encoded secret key
///
/// # Returns
/// * `String` - Base64 encoded string containing ciphertext and nonce
///
/// # Panics
/// * Panics if the secret cannot be decoded
/// * Panics if encryption fails
pub fn encrypt_password(password: &str, secret: String) -> String {
    let secret_decoded = general_purpose::STANDARD.decode(secret);
    if secret_decoded.is_err() {
        error!("Error decoding use secret");
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
            error!("An unexpected error occured");
        }
        let ciphertext_encoded = general_purpose::STANDARD.encode(ciphertext.unwrap());
        let nonce_encoded = general_purpose::STANDARD.encode(nonce_bytes);

        return format!("{}.{}", ciphertext_encoded, nonce_encoded);
    } else {
        error!("An unexpected error occured");
    }
}

#[derive(Error, Debug)]
pub enum DecryptionReturnError {
    #[error("Decryption error: {0}")]
    DecryptionError(String),

    #[error("Invalid UTF-8 sequence: {0}")]
    Utf8Error(#[from] FromUtf8Error), // Maps UTF-8 conversion errors
}

impl From<aes_gcm::Error> for DecryptionReturnError {
    fn from(err: aes_gcm::Error) -> Self {
        DecryptionReturnError::DecryptionError(format!("{:?}", err))
    }
}

/// Decrypt a password using AES-256-GCM
///
/// This function decrypts a password that was encrypted using AES-256-GCM encryption.
/// The input string should be a base64 encoded string containing both the ciphertext
/// and nonce, separated by a dot.
///
/// # Arguments
/// * `password` - The encrypted password string (ciphertext.nonce)
/// * `secret` - The base64 encoded secret key
///
/// # Returns
/// * `Result<String, DecryptionReturnError>` - The decrypted password if successful
///
/// # Errors
/// * Returns `DecryptionReturnError` if decryption fails
/// * Returns `DecryptionReturnError` if the password format is invalid
/// * Returns `DecryptionReturnError` if UTF-8 conversion fails
pub fn decrypt_password(password: String, secret: String) -> Result<String, DecryptionReturnError> {
    let secret_decoded = general_purpose::STANDARD.decode(secret);
    if secret_decoded.is_err() {
        error!("Error decoding use secret");
    }
    let secret_decoded = secret_decoded.unwrap();

    let cipher = Aes256Gcm::new_from_slice(&secret_decoded);
    if cipher.is_err() {
        error!("Error decoding use secret");
    }
    let cipher = cipher.unwrap();

    // Split password and nonce
    let password_split = password.split(".").collect::<Vec<&str>>();
    let ciphertext = general_purpose::STANDARD.decode(password_split[0]);
    let nonce = general_purpose::STANDARD.decode(password_split[1]);

    if nonce.is_err() || ciphertext.is_err() {
        error!("Unable to decode password");
    }

    // Decrypt the password
    let decrypted_bytes = cipher.decrypt(
        Nonce::from_slice(&nonce.unwrap()),
        ciphertext.unwrap().as_slice(),
    )?;

    Ok(String::from_utf8(decrypted_bytes)?)
}
