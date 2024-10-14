use colored::Colorize;

pub fn title(title: &str) {
    println!("{} {} {} {}", "Connected to database".cyan(), "PLACEHOLDER", "on".cyan(), "PLACEHOLDER");
    println!("\n{} {} {}\n", "-----".green(), title, "-----".green());
}
