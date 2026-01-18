use anylist_rs::{
    lists::{List, ListItem},
    AnyListClient,
};
use clap::{Arg, ArgMatches, Command};

use crate::auth::read_tokens;
use crate::error::CliError;

fn display_list_items(list: &List) {
    println!("\n{}", list.name());
    println!("{}", "=".repeat(list.name().len()));
    println!();

    // Display unchecked items
    let unchecked_items: Vec<&ListItem> = list.items().iter().filter(|item| !item.is_checked()).collect();
    if !unchecked_items.is_empty() {
        let mut sorted: Vec<&ListItem> = unchecked_items;
        sorted.sort_by(|a, b| a.name().to_lowercase().cmp(&b.name().to_lowercase()));

        for item in sorted {
            print!("  [ ] \x1B[1m{}\x1B[0m ({})", item.name(), item.id());
            if let Some(qty) = &item.quantity() {
                print!(" ({})", qty);
            }
            if !item.details().is_empty() {
                print!(" - {}", item.details());
            }
            if let Some(cat) = &item.category() {
                print!(" [{}]", cat);
            }
            println!();
        }
    } else {
        println!("  (no items)");
    }

    // Display checked items
    let checked_items: Vec<&ListItem> = list.items().iter().filter(|item| item.is_checked()).collect();
    if !checked_items.is_empty() {
        println!("\nCompleted:");
        for item in checked_items {
            print!("  [✓] {}", item.name());
            if let Some(qty) = &item.quantity() {
                print!(" ({})", qty);
            }
            println!();
        }
    }
    println!();
}

fn display_lists_names(lists: Vec<List>) {
    if lists.is_empty() {
        println!("No lists found.");
        return;
    }

    println!("\nYour Lists:");
    println!("{}", "=".repeat(11));
    println!();
    for list in lists {
        let item_count = list.items().iter().filter(|item| !item.is_checked()).count();
        println!("  • {} ({} items)", list.name(), item_count);
    }
    println!();
}

pub fn command() -> Command {
    Command::new("list")
        .about("View and manage your AnyList lists")
        .long_about(
            "View and manage your AnyList lists.\n\n\
             By default, this command shows all your lists with item counts.\n\
             Use subcommands to view, create, rename, or delete lists.",
        )
        .subcommand(
            Command::new("get")
                .about("Display items in a specific list")
                .arg(
                    Arg::new("name")
                        .help("Name of the list to display")
                        .required(true)
                        .value_name("LIST_NAME"),
                ),
        )
        .subcommand(
            Command::new("create")
                .about("Create a new shopping list")
                .arg(Arg::new("name").required(true).help("List name")),
        )
        .subcommand(
            Command::new("rename")
                .about("Rename a shopping list")
                .arg(Arg::new("name").required(true).help("Current list name"))
                .arg(Arg::new("new_name").required(true).help("New list name")),
        )
        .subcommand(
            Command::new("delete")
                .about("Delete a shopping list")
                .arg(Arg::new("name").required(true).help("List name to delete")),
        )
}

pub async fn exec_command(matches: &ArgMatches) -> Result<(), CliError> {
    let tokens = read_tokens()?;
    let client = AnyListClient::from_tokens(tokens)?;

    match matches.subcommand() {
        Some(("get", sub_matches)) => {
            let list_name = sub_matches
                .get_one::<String>("name")
                .expect("required argument");
            let list = client.get_list_by_name(list_name).await?;
            display_list_items(&list);
        }
        Some(("create", sub_matches)) => {
            let name = sub_matches.get_one::<String>("name").unwrap();
            let list = client.create_list(name).await?;
            println!("Created list: {} (ID: {})", list.name(), list.id());
        }
        Some(("rename", sub_matches)) => {
            let name = sub_matches.get_one::<String>("name").unwrap();
            let new_name = sub_matches.get_one::<String>("new_name").unwrap();

            let list = client.get_list_by_name(name).await?;
            client.rename_list(&list.id(), new_name).await?;
            println!("Renamed list '{}' to '{}'", name, new_name);
        }
        Some(("delete", sub_matches)) => {
            let name = sub_matches.get_one::<String>("name").unwrap();

            let list = client.get_list_by_name(name).await?;
            client.delete_list(&list.id()).await?;
            println!("Deleted list '{}'", name);
        }
        _ => {
            let lists = client.get_lists().await?;
            display_lists_names(lists);
        }
    }

    Ok(())
}
