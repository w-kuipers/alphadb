use crate::commands::connect::Connection;
use crate::config::setup::{get_home, ALPHADB_DIR, CONFIG_DIR, SESSIONS_FILE};
use crate::utils::{encrypt_password, error};
use alphadb::AlphaDB;
use colored::Colorize;
use inquire::{required, CustomType, Password, Text};
use serde::Deserialize;
use serde_derive::Serialize;
use std::{collections::BTreeMap, fs};
use toml;

#[derive(Debug, Default, Serialize, Deserialize)]
struct DbSessions {
    sessions: BTreeMap<String, Session>,
    setup: Setup,
}


#[derive(Debug, Default, Serialize, Deserialize)]
struct Setup {
    active_session: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Session {
    host: String,
    user: String,
    password: String,
    database: String,
    port: u16,
}

pub fn new_connection(activate: bool) -> String {
    let home = get_home();

    print!("\n");
    let host = Text::new("Host")
        .with_default("localhost")
        .with_help_message("URL/IP")
        .prompt()
        .unwrap();

    let user = Text::new("User")
        .with_validator(required!("This field is required"))
        .with_help_message("User with permissions to alter the database")
        .prompt()
        .unwrap();

    let password = Password::new("Password")
        .with_custom_confirmation_error_message("Passwords do not match")
        .with_validator(required!("This field is required"))
        .prompt()
        .unwrap();

    let database = Text::new("Database")
        .with_validator(required!("This field is required"))
        .with_help_message("Name of the database to connect to")
        .prompt()
        .unwrap();

    let port: u16 = CustomType::new("Port")
        .with_error_message("Port should be a number")
        .with_default(3306)
        .prompt()
        .unwrap();

    let connection = Connection {
        host,
        user,
        password,
        database,
        port,
    };

    // Try if the credentials will connect
    let mut db = AlphaDB::new();
    let testconn = db.connect(
        &connection.host,
        &connection.user,
        &connection.password,
        &connection.database,
        &connection.port,
    );

    if testconn.is_err() {
        error(testconn.unwrap_err().to_string());
    }

    println!(
        "\n{}\n",
        "Successfully able to connect to the database".green()
    );

    let label: String = CustomType::new("Label")
        .with_help_message("Optionally add a label to this connection")
        .with_default(format!("{}@{}", &connection.database, &connection.host))
        .prompt()
        .unwrap();

    let mut file = DbSessions::default();
    file.sessions.insert(
        label.to_string(),
        Session {
            host: connection.host,
            user: connection.user,
            password: encrypt_password(&connection.password),
            database: connection.database,
            port: connection.port,
        },
    );
    file.setup.active_session.insert(label.to_string());

    let toml_string = match toml::to_string(&file) {
        Ok(c) => c,
        Err(_) => {
            error(format!(
                "An unexpected error occured. Unable to encode generated config."
            ));
        }
    };
    let sessions_file = home.join(CONFIG_DIR).join(ALPHADB_DIR).join(SESSIONS_FILE);

    match fs::write(&sessions_file, toml_string) {
        Ok(c) => c,
        Err(_) => {
            error(format!(
                "Unable to write to config file: '{}'",
                sessions_file.display().to_string().blue(),
            ));
        }
    };

    return label;
}

pub fn get_connections() -> Option<Vec<String>> {
    let home = get_home();
    let config_dir = home.join(CONFIG_DIR).join(ALPHADB_DIR);
    let sessions_file = config_dir.join(SESSIONS_FILE);

    let sessions_content_raw = match fs::read_to_string(&sessions_file) {
        Ok(c) => c,
        Err(_) => {
            return None;
        }
    };

    if sessions_content_raw.is_empty() {
        return None;
    }

    let sessions_content: DbSessions = match toml::from_str(&sessions_content_raw) {
        Ok(c) => c,
        Err(_) => {
            error(format!(
                "Unable to deserialize config file: '{}' is it corrupted?",
                sessions_file.display().to_string().blue(),
            ));
        }
    };

    let mut connections = Vec::new();

    for connection in sessions_content.sessions {
        connections.push(connection.0);
    }

    return Some(connections);
}

pub fn set_active_connection() {
    let home = get_home();
    let config_dir = home.join(CONFIG_DIR).join(ALPHADB_DIR);
    let sessions_file = config_dir.join(SESSIONS_FILE);


    let sessions_content_raw = match fs::read_to_string(&sessions_file) {
        Ok(c) => c,
        Err(_) => {
             
        }
    };
}
