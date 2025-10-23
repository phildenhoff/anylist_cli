extern crate clap;

use anylist_rs::AnyListClient;
use clap::{ArgMatches, Command, SubCommand};
use inquire::{Password, Text};

use crate::auth::save_credentials;

pub fn command() -> Command<'static> {
    return SubCommand::with_name("login");
}

pub async fn exec_command(_matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let email = Text::new("Email: ").prompt().unwrap();
    let password = Password::new("Password: ")
        .without_confirmation()
        .prompt()
        .unwrap();

    let result = AnyListClient::login(&email, &password).await;

    match result {
        Ok(client) => {
            println!("You're signed in! You can start using {} to read, create, and update your grocery lists!", env!("CARGO_BIN_NAME"));
            println!("\nFor more info, see {} --help", env!("CARGO_BIN_NAME"));

            save_credentials(client)?;
        }

        Err(err) => {
          dbg!(err);

          println!("Login failed.")
        }
    }

    Ok(())
}
