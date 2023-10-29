mod commands;

extern crate clap;

use anylist_client::lists::get_lists;
use anylist_client::login;
use clap::Command;
use commands::login::login_subcommand;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("AnyList Client")
        .subcommand(login_subcommand())
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("login") {
        let email = matches.value_of("email").unwrap();
        let password = matches.value_of("password").unwrap();
        login::login(email, password).await?;
    }

    get_lists("2|1:0|10:1698566722|14:signed_user_id|44:Y2RhMjFiMDA3ODY0NGEwMWI2NDBjODRkM2Q3NDE4N2U=|de1b16f32bbe046dd032bc6252a40cb5fda397004fa086c0db314169539cbf5a").await?;

    Ok(())
}
