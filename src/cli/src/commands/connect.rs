use crate::utils::title;
use inquire::{required, CustomType, Password, Text};

pub fn connect() {
    title("Connect");

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
}
