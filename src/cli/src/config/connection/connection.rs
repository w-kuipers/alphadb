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

use crate::config::setup::{get_config_content, write_config};
use crate::error;
use colored::Colorize;
use serde::Deserialize;
use serde_derive::Serialize;
use std::collections::BTreeMap;

#[derive(Debug, Serialize, Deserialize)]
pub enum SessionType {
    Postgres(PostgresSession),
    Mysql(MysqlSession),
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DbSessions {
    pub sessions: BTreeMap<String, SessionType>,
    pub setup: Setup,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Setup {
    pub active_session: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MysqlSession {
    pub host: String,
    pub user: String,
    pub password: String,
    pub database: String,
    pub port: u16,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PostgresSession {
    pub host: String,
    pub user: String,
    pub password: String,
    pub database: String,
    pub port: u16,
}

/// Get all saved database connections
///
/// This function retrieves all saved connections from the sessions config file.
/// The active connection, if any, will be marked with "(active)" in green.
///
/// # Returns
/// * `Option<Vec<String>>` - List of connection labels if any exist, None otherwise
pub fn get_connections() -> Option<Vec<String>> {
    let sessions_content = match get_config_content::<DbSessions>() {
        Some(s) => s,
        None => return None,
    };

    let mut connections = Vec::new();
    for connection in sessions_content.sessions {
        let mut label = connection.0.clone();
        if let Some(active_session) = sessions_content.setup.active_session.clone() {
            if connection.0 == active_session {
                label = format!("{} {}", connection.0, "(active)".green())
            }
        }
        connections.push(label);
    }

    return Some(connections);
}

#[derive(Debug)]
pub struct ActiveConnection {
    pub label: String,
    pub connection: SessionType,
}

/// Get the currently active database connection
///
/// # Returns
/// * `Option<ActiveConnection>` - The active connection if one exists, None otherwise
pub fn get_active_connection() -> Option<ActiveConnection> {
    let mut sessions_content = match get_config_content::<DbSessions>() {
        Some(s) => s,
        None => return None,
    };

    let active_session = match sessions_content.setup.active_session {
        Some(a) => a,
        None => return None,
    };

    let connection = match sessions_content.sessions.remove(&active_session) {
        Some(c) => c,
        None => return None,
    };

    return Some(ActiveConnection {
        label: active_session,
        connection: connection,
    });
}

/// Set a connection as the active database connection
///
/// # Arguments
/// * `label` - The label of the connection to set as active
///
/// # Panics
/// * Panics if unable to write to the config file
pub fn set_active_connection(label: &String) {
    let mut sessions_content = match get_config_content::<DbSessions>() {
        Some(c) => c,
        None => {
            error!("No sessions found".to_string());
        }
    };

    sessions_content
        .setup
        .active_session
        .insert(label.to_string());

    write_config(sessions_content);
}

/// Remove a saved database connection
///
/// # Arguments
/// * `label` - The label of the connection to remove
///
/// # Panics
/// * Panics if no sessions exist
/// * Panics if unable to write to the config file
pub fn remove_connection(label: String) {
    let mut sessions_content = match get_config_content::<DbSessions>() {
        Some(c) => c,
        None => {
            error!("No sessions found".to_string());
        }
    };

    sessions_content.sessions.remove(&label);

    if let Some(active_session) = &sessions_content.setup.active_session {
        if active_session == &label {
            sessions_content.setup.active_session = None;
        }
    }

    write_config(sessions_content);
}
