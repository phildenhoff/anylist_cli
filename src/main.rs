mod auth;
mod commands;

extern crate clap;

use clap::Command;
use commands::{list, login};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut commands = Command::new("anylist")
        .about("See, update, and add to your AnyList lists.")
        .subcommand(login::command())
        .subcommand(list::command());
    let matches = commands.clone().get_matches();

    match matches.subcommand() {
        Some(("login", matches)) => {
            login::exec_command(matches).await?;
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
