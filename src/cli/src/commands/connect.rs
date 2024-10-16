use crate::config::{get_connections, save_connection};
use crate::utils::{error, title};
use alphadb::AlphaDB;
use colored::Colorize;
use inquire::{required, CustomType, Password, Select, Text};

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

            save_connection(connection, &label);

            println!(
                "\n{} {} {}\n",
                "Database connection".green(),
                label.cyan(),
                "saved and ready for use.".green()
            );
        }
    }
}
