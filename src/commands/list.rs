use anylist_rs::{
    lists::{List, ListItem},
    AnyListClient,
};
use clap::{Arg, ArgMatches, Command};

use crate::auth::read_tokens;
use crate::error::CliError;

struct FlagIds;
impl FlagIds {
    const LIST_NAME: &'static str = "list_name";
}

fn display_list_items(list_name: &str, lists: Vec<List>) -> Result<(), CliError> {
    let list = lists
        .iter()
        .find(|list| list.name.eq_ignore_ascii_case(list_name))
        .ok_or_else(|| CliError::ListNotFound(list_name.to_string()))?;

    println!("\n{}", list.name);
    println!("{}", "=".repeat(list.name.len()));

    let mut unchecked_items: Vec<&ListItem> = list
        .items
        .iter()
        .filter(|item| !item.is_checked)
        .collect();

    unchecked_items.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    if unchecked_items.is_empty() {
        println!("\n  (no items)");
    } else {
        println!();
        for item in unchecked_items {
            if item.details.is_empty() {
                println!("  • {}", item.name);
            } else {
                println!("  • {}: {}", item.name, item.details);
            }
        }
    }
    println!();

    Ok(())
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
        let item_count = list.items.iter().filter(|item| !item.is_checked).count();
        println!("  • {} ({} items)", list.name, item_count);
    }
    println!();
}

pub fn command() -> Command {
    Command::new("list")
        .about("View and manage your AnyList lists")
        .long_about(
            "View and manage your AnyList lists.\n\n\
             By default, this command shows all your lists with item counts.\n\
             Use subcommands to view specific list details.",
        )
        .subcommand(
            Command::new("get")
                .about("Display items in a specific list")
                .arg(
                    Arg::new(FlagIds::LIST_NAME)
                        .help("Name of the list to display")
                        .required(true)
                        .value_name("LIST_NAME"),
                ),
        )
}

pub async fn exec_command(matches: &ArgMatches) -> Result<(), CliError> {
    let tokens = read_tokens()?;
    let client = AnyListClient::from_tokens(tokens)?;

    let lists = client.get_lists().await?;

    match matches.subcommand() {
        Some(("get", sub_matches)) => {
            let list_name = sub_matches
                .get_one::<String>(FlagIds::LIST_NAME)
                .expect("required argument");

            display_list_items(list_name, lists)?;
        }
        _ => {
            display_lists_names(lists);
        }
    }

    Ok(())
}
