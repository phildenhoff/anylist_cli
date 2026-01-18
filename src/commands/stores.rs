use anylist_rs::AnyListClient;
use clap::{Arg, ArgMatches, Command};

use crate::auth::read_tokens;
use crate::error::CliError;

pub fn command() -> Command {
    Command::new("store")
        .about("Manage stores for lists")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("list")
                .about("List all stores for a list")
                .arg(Arg::new("list").required(true).help("List name or ID")),
        )
        .subcommand(
            Command::new("add")
                .about("Add a store to a list")
                .arg(Arg::new("list").required(true).help("List name or ID"))
                .arg(Arg::new("name").required(true).help("Store name")),
        )
        .subcommand(
            Command::new("update")
                .about("Update a store's name")
                .arg(Arg::new("list").required(true).help("List name or ID"))
                .arg(Arg::new("store_id").required(true).help("Store ID"))
                .arg(Arg::new("name").required(true).help("New store name")),
        )
        .subcommand(
            Command::new("delete")
                .about("Delete a store from a list")
                .arg(Arg::new("list").required(true).help("List name or ID"))
                .arg(Arg::new("store_id").required(true).help("Store ID")),
        )
}

pub async fn exec_command(matches: &ArgMatches) -> Result<(), CliError> {
    let tokens = read_tokens()?;
    let client = AnyListClient::from_tokens(tokens)?;

    match matches.subcommand() {
        Some(("list", sub_matches)) => {
            let list_name = sub_matches.get_one::<String>("list").unwrap();

            let list = client.get_list_by_name(list_name).await?;
            let stores = client.get_stores_for_list(&list.id()).await?;

            if stores.is_empty() {
                println!("No stores found for list '{}'", list.name());
            } else {
                println!("Stores for list '{}':", list.name());
                for store in stores {
                    println!("  {} - {}", store.id(), store.name());
                }
            }
        }
        Some(("add", sub_matches)) => {
            let list_name = sub_matches.get_one::<String>("list").unwrap();
            let name = sub_matches.get_one::<String>("name").unwrap();

            let list = client.get_list_by_name(list_name).await?;
            let store = client.create_store(&list.id(), name).await?;

            println!("Created store '{}' for list '{}'", store.name(), list.name());
        }
        Some(("update", sub_matches)) => {
            let list_name = sub_matches.get_one::<String>("list").unwrap();
            let store_id = sub_matches.get_one::<String>("store_id").unwrap();
            let name = sub_matches.get_one::<String>("name").unwrap();

            let list = client.get_list_by_name(list_name).await?;
            client.update_store(&list.id(), store_id, name).await?;

            println!("Updated store to '{}' in list '{}'", name, list.name());
        }
        Some(("delete", sub_matches)) => {
            let list_name = sub_matches.get_one::<String>("list").unwrap();
            let store_id = sub_matches.get_one::<String>("store_id").unwrap();

            let list = client.get_list_by_name(list_name).await?;
            client.delete_store(&list.id(), store_id).await?;

            println!("Deleted store from list '{}'", list.name());
        }
        _ => unreachable!("subcommand_required prevents this"),
    }

    Ok(())
}
