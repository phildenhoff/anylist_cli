extern crate clap;

use anylist_rs::login;
use clap::{ArgMatches, Command, SubCommand};
use inquire::{Password, Text};

pub fn command() -> Command<'static> {
    return SubCommand::with_name("login")
}

pub async fn exec_command(_matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let email = Text::new("Email: ").prompt().unwrap();
    let password = Password::new("Password: ")
        .without_confirmation()
        .prompt()
        .unwrap();

    login::login(email.as_str(), password.as_str()).await?;
    Ok(())
}
