mod os;
mod file;

use std::{fs, io::{self, Write}};

#[tokio::main]
async fn main() {
    os::clear();

    let mut choice: String = String::new();

    print!("Please specify what you want to do [install, remove]: ");

    io::stdout().flush().expect("Failed to flush console output");

    io::stdin().read_line(&mut choice).expect("Failed to get user choice");

    choice = choice.replace("\r", "").replace("\n", "");

    match choice.to_lowercase().as_str() {
        "install" => {
            os::clear();

            file::download().await;
        },
        "remove" => {
            os::clear();

            println!("You choose to remove project-generator.")
        },
        _ => {
            eprintln!("Please enter a valid input");
            main();
        },
    }
}
