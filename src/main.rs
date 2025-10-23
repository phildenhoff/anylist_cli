mod auth;
mod commands;

extern crate clap;

use clap::Command;
use commands::{categories, items, list, login, meal_plans, stores};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut commands = Command::new("anylist")
        .about("Manage your AnyList shopping lists, items, meal plans, and more.")
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand(login::command())
        .subcommand(list::command())
        .subcommand(items::command())
        .subcommand(stores::command())
        .subcommand(categories::command())
        .subcommand(meal_plans::command());
    let matches = commands.clone().get_matches();

    match matches.subcommand() {
        Some(("login", matches)) => {
            login::exec_command(matches).await?;
        }
        Some(("list", matches)) => {
            list::exec_command(matches).await?;
        }
        Some(("item", matches)) => {
            items::exec_command(matches).await?;
        }
        Some(("store", matches)) => {
            stores::exec_command(matches).await?;
        }
        Some(("category", matches)) => {
            categories::exec_command(matches).await?;
        }
        Some(("meal-plan", matches)) => {
            meal_plans::exec_command(matches).await?;
        }
        _ => {
            commands.print_help()?;
        }
    }
    Ok(())
}
