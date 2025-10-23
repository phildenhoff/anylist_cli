use anylist_rs::AnyListClient;
use clap::{ArgMatches, Command};
use inquire::{Password, Text};

use crate::auth::save_credentials;
use crate::error::CliError;

pub fn command() -> Command {
    Command::new("login")
        .about("Login to your AnyList account")
}

pub async fn exec_command(_matches: &ArgMatches) -> Result<(), CliError> {
    let email = Text::new("Email:")
        .prompt()?;

    let password = Password::new("Password:")
        .with_display_mode(inquire::PasswordDisplayMode::Masked)
        .without_confirmation()
        .prompt()?;

    match AnyListClient::login(&email, &password).await {
        Ok(client) => {
            save_credentials(client)?;
            println!("✓ Successfully logged in!");
            println!("\nYou can now use {} to manage your AnyList lists.", env!("CARGO_BIN_NAME"));
            println!("Try: {} list", env!("CARGO_BIN_NAME"));
            Ok(())
        }
        Err(err) => {
            Err(CliError::LoginFailed(format!("Authentication failed: {}", err)))
        }
    }
}
