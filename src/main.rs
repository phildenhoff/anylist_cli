mod commands;

extern crate clap;

use anylist_rs::login;
use clap::Command;
use commands::list;
use commands::login::login_subcommand;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut commands = Command::new("anylist")
        .about("See, update, and add to your AnyList lists.")
        .subcommand(login_subcommand())
        .subcommand(list::command());
    let matches = commands.clone().get_matches();

    match matches.subcommand() {
        Some(("login", matches)) => {
            let email = matches.value_of("email").unwrap();
            let password = matches.value_of("password").unwrap();
            login::login(email, password).await?;
        }
        Some(("list", matches)) => {
            list::exec_command(matches).await?;
        }
        _ => {
            commands.print_help()?;
        }
    }
    Ok(())
}
