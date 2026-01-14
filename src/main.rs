mod auth;
mod commands;
mod error;

use clap::Command;
use commands::{categories, items, list, login, meal_plans, recipes, stores, tail};
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
        .about("Manage your AnyList shopping lists, items, recipes, meal plans, and more.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(login::command())
        .subcommand(list::command())
        .subcommand(items::command())
        .subcommand(stores::command())
        .subcommand(categories::command())
        .subcommand(meal_plans::command())
        .subcommand(recipes::command())
        .subcommand(tail::command())
        .get_matches();

    match matches.subcommand() {
        Some(("login", sub_matches)) => {
            login::exec_command(sub_matches).await?;
        }
        Some(("list", sub_matches)) => {
            list::exec_command(sub_matches).await?;
        }
        Some(("item", sub_matches)) => {
            items::exec_command(sub_matches).await?;
        }
        Some(("store", sub_matches)) => {
            stores::exec_command(sub_matches).await?;
        }
        Some(("category", sub_matches)) => {
            categories::exec_command(sub_matches).await?;
        }
        Some(("meal-plan", sub_matches)) => {
            meal_plans::exec_command(sub_matches).await?;
        }
        Some(("recipe", sub_matches)) => {
            recipes::exec_command(sub_matches).await?;
        }
        Some(("tail", sub_matches)) => {
            tail::exec_command(sub_matches).await?;
        }
        _ => unreachable!("clap should prevent this due to subcommand_required(true)"),
    }

    Ok(())
}
