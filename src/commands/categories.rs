use anylist_rs::AnyListClient;
use clap::{Arg, ArgMatches, Command};

use crate::auth::read_tokens;
use crate::error::CliError;

pub fn command() -> Command {
    Command::new("category")
        .about("Manage categories for list items")
        .subcommand(
            Command::new("add")
                .about("Add a category to a list")
                .arg(Arg::new("list").required(true).help("List name or ID"))
                .arg(
                    Arg::new("category_group_id")
                        .required(true)
                        .help("Category group ID"),
                )
                .arg(Arg::new("name").required(true).help("Category name")),
        )
        .subcommand(
            Command::new("rename")
                .about("Rename a category")
                .arg(Arg::new("list").required(true).help("List name or ID"))
                .arg(
                    Arg::new("category_group_id")
                        .required(true)
                        .help("Category group ID"),
                )
                .arg(Arg::new("category_id").required(true).help("Category ID"))
                .arg(Arg::new("name").required(true).help("New category name")),
        )
        .subcommand(
            Command::new("delete")
                .about("Delete a category from a list")
                .arg(Arg::new("list").required(true).help("List name or ID"))
                .arg(Arg::new("category_id").required(true).help("Category ID")),
        )
}

pub async fn exec_command(matches: &ArgMatches) -> Result<(), CliError> {
    let tokens = read_tokens()?;
    let client = AnyListClient::from_tokens(tokens)?;

    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            let list_name = sub_matches.get_one::<String>("list").unwrap();
            let category_group_id = sub_matches.get_one::<String>("category_group_id").unwrap();
            let name = sub_matches.get_one::<String>("name").unwrap();

            let list = client.get_list_by_name(list_name).await?;
            let category = client
                .create_category(&list.id, category_group_id, name)
                .await?;

            println!(
                "Created category '{}' in list '{}'",
                category.name, list.name
            );
        }
        Some(("rename", sub_matches)) => {
            let list_name = sub_matches.get_one::<String>("list").unwrap();
            let category_group_id = sub_matches.get_one::<String>("category_group_id").unwrap();
            let category_id = sub_matches.get_one::<String>("category_id").unwrap();
            let name = sub_matches.get_one::<String>("name").unwrap();

            let list = client.get_list_by_name(list_name).await?;
            client
                .rename_category(&list.id, category_group_id, category_id, name)
                .await?;

            println!("Renamed category to '{}' in list '{}'", name, list.name);
        }
        Some(("delete", sub_matches)) => {
            let list_name = sub_matches.get_one::<String>("list").unwrap();
            let category_id = sub_matches.get_one::<String>("category_id").unwrap();

            let list = client.get_list_by_name(list_name).await?;
            client.delete_category(&list.id, category_id).await?;

            println!("Deleted category from list '{}'", list.name);
        }
        _ => {
            println!("Unknown category command. Use --help for usage.");
        }
    }

    Ok(())
}
