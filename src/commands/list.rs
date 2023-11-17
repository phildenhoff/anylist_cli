use anylist_rs::lists::{self, List, ListItem};
use clap::{Arg, ArgMatches, Command};

use crate::auth::get_signed_user_id;

struct FlagIds;
impl FlagIds {
    const LIST_NAME: &'static str = "list_name";
}

fn display_list_items(list_name: &String, lists: Vec<List>) -> () {
    for list in lists {
        if list.name.to_lowercase() == list_name.to_lowercase() {
            // e.g.
            // Groceries
            // ---------
            println!("{}", list.name);
            for _ in 0..list.name.len() {
                print!("-");
            }
            println!("\n");

            let unchecked_items = list.items.iter().filter(|item| !item.is_checked);
            let mut sorted: Vec<&ListItem> = unchecked_items.collect();
            sorted.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

            for item in sorted {
                // bold, using ANSI escape codes
                println!("\x1B[1m{}\x1B[0m: {}", item.name, item.details);
            }
        }
    }
}

fn display_lists_names(lists: Vec<List>) -> () {
    for list in lists {
        println!("{}", list.name);
    }
}

pub fn command() -> Command<'static> {
    return Command::new("list")
        .about("Perform actions on lists.")
        .long_about(
            "
        By default, this command will print out all of your lists and their items.
        
        You can use the subcommands to perform other actions, like getting the
        elements of one list, or adding an item to a list.
        ",
        )
        .subcommand(
            Command::new("get")
                .about("Get a list.")
                .arg(Arg::new("list_name").id(FlagIds::LIST_NAME).required(true)),
        );
}

pub async fn exec_command(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let signed_user_id = get_signed_user_id().unwrap();
    let lists = lists::get_lists(signed_user_id.as_str()).await?;

    match matches.subcommand() {
        Some(("get", sub_matches)) => {
            if let Some(list_name) = sub_matches.get_one::<String>(FlagIds::LIST_NAME) {
                display_list_items(list_name, lists)
            }
        }
        _ => {
            display_lists_names(lists);
        }
    }

    Ok(())
}
