use anylist_rs::AnyListClient;
use clap::{Arg, ArgMatches, Command};

use crate::auth::read_tokens;

pub fn command() -> Command<'static> {
    Command::new("item")
        .about("Manage items in shopping lists")
        .subcommand(
            Command::new("add")
                .about("Add an item to a list")
                .arg(Arg::new("list").required(true).help("List name or ID"))
                .arg(Arg::new("name").required(true).help("Item name"))
                .arg(
                    Arg::new("quantity")
                        .short('q')
                        .long("quantity")
                        .takes_value(true)
                        .help("Item quantity (e.g., '2 lbs', '500g')"),
                )
                .arg(
                    Arg::new("details")
                        .short('d')
                        .long("details")
                        .takes_value(true)
                        .help("Additional details or notes"),
                )
                .arg(
                    Arg::new("category")
                        .short('c')
                        .long("category")
                        .takes_value(true)
                        .help("Category name"),
                ),
        )
        .subcommand(
            Command::new("update")
                .about("Update an existing item")
                .arg(Arg::new("list").required(true).help("List name or ID"))
                .arg(Arg::new("item_id").required(true).help("Item ID"))
                .arg(Arg::new("name").required(true).help("New item name"))
                .arg(
                    Arg::new("quantity")
                        .short('q')
                        .long("quantity")
                        .takes_value(true)
                        .help("New quantity"),
                )
                .arg(
                    Arg::new("details")
                        .short('d')
                        .long("details")
                        .takes_value(true)
                        .help("New details"),
                )
                .arg(
                    Arg::new("category")
                        .short('c')
                        .long("category")
                        .takes_value(true)
                        .help("New category"),
                ),
        )
        .subcommand(
            Command::new("delete")
                .about("Delete an item from a list")
                .arg(Arg::new("list").required(true).help("List name or ID"))
                .arg(Arg::new("item_id").required(true).help("Item ID")),
        )
        .subcommand(
            Command::new("check")
                .about("Check/cross off an item")
                .arg(Arg::new("list").required(true).help("List name or ID"))
                .arg(Arg::new("item_id").required(true).help("Item ID")),
        )
        .subcommand(
            Command::new("uncheck")
                .about("Uncheck an item")
                .arg(Arg::new("list").required(true).help("List name or ID"))
                .arg(Arg::new("item_id").required(true).help("Item ID")),
        )
}

pub async fn exec_command(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let tokens = read_tokens()?;
    let client = AnyListClient::from_tokens(tokens)?;

    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            let list_name = sub_matches.get_one::<String>("list").unwrap();
            let name = sub_matches.get_one::<String>("name").unwrap();
            let quantity = sub_matches.get_one::<String>("quantity").map(|s| s.as_str());
            let details = sub_matches.get_one::<String>("details").map(|s| s.as_str());
            let category = sub_matches.get_one::<String>("category").map(|s| s.as_str());

            let list = client.get_list_by_name(list_name).await?;
            let item = client
                .add_item_with_details(&list.id, name, quantity, details, category)
                .await?;

            println!("Added item '{}' to list '{}'", item.name, list.name);
            if let Some(q) = &item.quantity {
                println!("  Quantity: {}", q);
            }
            if !item.details.is_empty() {
                println!("  Details: {}", item.details);
            }
            if let Some(c) = &item.category {
                println!("  Category: {}", c);
            }
        }
        Some(("update", sub_matches)) => {
            let list_name = sub_matches.get_one::<String>("list").unwrap();
            let item_id = sub_matches.get_one::<String>("item_id").unwrap();
            let name = sub_matches.get_one::<String>("name").unwrap();
            let quantity = sub_matches.get_one::<String>("quantity").map(|s| s.as_str());
            let details = sub_matches.get_one::<String>("details").map(|s| s.as_str());
            let category = sub_matches.get_one::<String>("category").map(|s| s.as_str());

            let list = client.get_list_by_name(list_name).await?;
            client
                .update_item(&list.id, item_id, name, quantity, details, category)
                .await?;

            println!("Updated item '{}' in list '{}'", name, list.name);
        }
        Some(("delete", sub_matches)) => {
            let list_name = sub_matches.get_one::<String>("list").unwrap();
            let item_id = sub_matches.get_one::<String>("item_id").unwrap();

            let list = client.get_list_by_name(list_name).await?;
            client.delete_item(&list.id, item_id).await?;

            println!("Deleted item from list '{}'", list.name);
        }
        Some(("check", sub_matches)) => {
            let list_name = sub_matches.get_one::<String>("list").unwrap();
            let item_id = sub_matches.get_one::<String>("item_id").unwrap();

            let list = client.get_list_by_name(list_name).await?;
            client.cross_off_item(&list.id, item_id).await?;

            println!("Checked off item in list '{}'", list.name);
        }
        Some(("uncheck", sub_matches)) => {
            let list_name = sub_matches.get_one::<String>("list").unwrap();
            let item_id = sub_matches.get_one::<String>("item_id").unwrap();

            let list = client.get_list_by_name(list_name).await?;
            client.uncheck_item(&list.id, item_id).await?;

            println!("Unchecked item in list '{}'", list.name);
        }
        _ => {
            println!("Unknown item command. Use --help for usage.");
        }
    }

    Ok(())
}
