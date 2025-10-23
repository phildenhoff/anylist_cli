mod auth;
mod commands;
mod error;

use clap::Command;
use commands::{list, login};
use error::CliError;
use std::process;

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
}

async fn run() -> Result<(), CliError> {
    let matches = Command::new("anylist")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Manage your AnyList lists from the command line")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(login::command())
        .subcommand(list::command())
        .get_matches();

    match matches.subcommand() {
        Some(("login", sub_matches)) => {
            login::exec_command(sub_matches).await?;
        }
        Some(("list", sub_matches)) => {
            list::exec_command(sub_matches).await?;
        }
        _ => unreachable!("clap should prevent this due to subcommand_required(true)"),
    }

    Ok(())
}
