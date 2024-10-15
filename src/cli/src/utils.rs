use colored::Colorize;
use std::process;

pub fn title(title: &str) {
    println!(
        "{} {} {} {}",
        "Connected to database".cyan(),
        "PLACEHOLDER",
        "on".cyan(),
        "PLACEHOLDER"
    );
    println!("\n{} {} {}\n", "-----".green(), title, "-----".green());
}

pub fn error(error_string: String) -> ! {
    // Some error messages are still wrapped in their definition
    let start = error_string.find("{").map(|pos| pos + 1).unwrap_or(0);
    let end = error_string.rfind("}").unwrap_or(error_string.len());

    let clean_error = &error_string[start + 1..end].trim();
    eprintln!("\n{}\n", clean_error.red());

    process::exit(1);
}
