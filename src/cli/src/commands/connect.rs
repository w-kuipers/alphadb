use crate::config::connection::{get_connections, new_connection};
use crate::utils::{error, title};
use colored::Colorize;
use inquire::Select;

pub struct Connection {
    pub host: String,
    pub user: String,
    pub password: String,
    pub database: String,
    pub port: u16,
}

pub fn connect() {
    title("Connect");

    if let Some(mut connections) = get_connections() {
        connections.push("++ New connection".to_string());

        let choice = Select::new("Choose a connection to set as active", connections)
            .with_vim_mode(true)
            .prompt();
        if choice.is_err() {
            error("An unexpected error occured".to_string());
        }

        let connection_choice = choice.unwrap();

        if connection_choice == "++ New connection".to_string() {
            let label = new_connection(true);

            println!(
                "\n{} {} {}\n",
                "Database connection".green(),
                label.cyan(),
                "saved and ready for use.".green()
            );
        }
    } else {
        let label = new_connection(true);

        println!(
            "\n{} {} {}\n",
            "Database connection".green(),
            label.cyan(),
            "saved and ready for use.".green()
        );
    }
}
