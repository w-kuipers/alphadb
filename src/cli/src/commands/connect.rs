use crate::config::connection::{get_connections, new_connection, set_active_connection};
use crate::utils::{error, title};
use colored::Colorize;
use inquire::Select;
use crate::config::setup::Config;

pub struct Connection {
    pub host: String,
    pub user: String,
    pub password: String,
    pub database: String,
    pub port: u16,
}

pub fn connect(config: &Config) {
    title("Connect");

    if let Some(mut connections) = get_connections() {
        connections.push("++ New connection".to_string());

        let choice = Select::new("Choose a connection to set as active", connections)
            .with_vim_mode(config.input.vim_bindings)
            .prompt();
        if choice.is_err() {
            error("An unexpected error occured".to_string());
        }

        let connection_choice = choice.unwrap();

        if connection_choice == "++ New connection".to_string() {
            let label = new_connection(true, config);

            println!(
                "\n{} {} {}\n",
                "Database connection".green(),
                label.cyan(),
                "saved and ready for use.".green()
            );
        } else {
            set_active_connection(&connection_choice);

            println!(
                "\n{} {} {}\n",
                "Database connection".green(),
                connection_choice.cyan(),
                "is now active".green()
            );
        }
    } else {
        let label = new_connection(true, config);

        println!(
            "\n{} {} {}\n",
            "Database connection".green(),
            label.cyan(),
            "saved and ready for use.".green()
        );
    }
}
