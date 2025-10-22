extern crate clap;

use anylist_rs::login;
use clap::{ArgMatches, Command, SubCommand};
use inquire::{Password, Text};

use crate::auth::save_login;

pub fn command() -> Command<'static> {
    return SubCommand::with_name("login");
}

pub async fn exec_command(_matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let email = Text::new("Email: ").prompt().unwrap();
    let password = Password::new("Password: ")
        .without_confirmation()
        .prompt()
        .unwrap();

    match login::login(email.as_str(), password.as_str()).await {
        Ok(result) => {
            println!("You're signed in! You can start using {} to read, create, and update your grocery lists!", env!("CARGO_BIN_NAME"));
            println!("\nFor more info, see {} --help", env!("CARGO_BIN_NAME"));
            save_login(result).unwrap();
        }

        Err(_) => println!("Login failed."),
    }

    Ok(())
}
