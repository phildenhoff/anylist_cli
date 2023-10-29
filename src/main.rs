mod commands;

extern crate clap;

use anylist_rs::lists::get_lists;
use anylist_rs::login;
use clap::{Arg, Command, SubCommand};
use commands::login::login_subcommand;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("AnyList Client")
        .subcommand(login_subcommand())
        .subcommand(
            SubCommand::with_name("lists")
                .about("Get the names of all lists")
                .arg(
                    Arg::new("signed_user_id")
                        .required(true)
                        .help("The signed user id"),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("login") {
        let email = matches.value_of("email").unwrap();
        let password = matches.value_of("password").unwrap();
        login::login(email, password).await?;
    }

    if let Some(matches) = matches.subcommand_matches("lists") {
        let signed_user_id = matches.value_of("signed_user_id").unwrap();
        let lists = get_lists(signed_user_id).await?;
        for list in lists {
            println!("List: {}", list.name);
            let all_item_names: String = list.items.iter().map(|item| item.name.as_str()).collect::<Vec<&str>>().join(", ");
            println!("\t{}", all_item_names);
        }
    }

    Ok(())
}
