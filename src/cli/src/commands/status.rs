use crate::config::connection::get_active_connection;
use crate::config::setup::Config;
use crate::utils::{decrypt_password, error, title};
use alphadb::AlphaDB;
use colored::Colorize;

pub fn status(config: &Config) {
    title("Status");

    if let Some(conn) = get_active_connection() {
        let mut db = AlphaDB::new();
        let password = decrypt_password(conn.password, config.main.secret.clone().unwrap());
        let connect = db.connect(
            &conn.host,
            &conn.user,
            &password,
            &conn.database,
            &conn.port,
        );

        if connect.is_err() {
            error(connect.err().unwrap().to_string());
        }

        // let status = db.status();
        // println!("{:?}", status);
    } else {
        println!("{}", "No database connection active".yellow());
    }
}
