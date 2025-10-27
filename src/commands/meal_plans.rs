use anylist_rs::AnyListClient;
use clap::{Arg, ArgMatches, Command};

use crate::auth::read_tokens;
use crate::error::CliError;

pub fn command() -> Command {
    Command::new("meal-plan")
        .about("Manage meal plan events")
        .subcommand(
            Command::new("list")
                .about("List meal plan events in a date range")
                .arg(
                    Arg::new("start_date")
                        .required(true)
                        .help("Start date (YYYY-MM-DD)"),
                )
                .arg(
                    Arg::new("end_date")
                        .required(true)
                        .help("End date (YYYY-MM-DD)"),
                ),
        )
        .subcommand(
            Command::new("add")
                .about("Add a meal plan event")
                .arg(Arg::new("calendar_id").required(true).help("Calendar ID"))
                .arg(Arg::new("date").required(true).help("Date (YYYY-MM-DD)"))
                .arg(
                    Arg::new("recipe_id")
                        .short('r')
                        .long("recipe-id")
                        .help("Recipe ID"),
                )
                .arg(
                    Arg::new("title")
                        .short('t')
                        .long("title")
                        .help("Event title (for non-recipe events)"),
                )
                .arg(
                    Arg::new("label_id")
                        .short('l')
                        .long("label-id")
                        .help("Meal label ID (Breakfast, Lunch, Dinner, etc.)"),
                ),
        )
        .subcommand(
            Command::new("update")
                .about("Update a meal plan event")
                .arg(Arg::new("calendar_id").required(true).help("Calendar ID"))
                .arg(Arg::new("event_id").required(true).help("Event ID"))
                .arg(Arg::new("date").required(true).help("New date (YYYY-MM-DD)"))
                .arg(
                    Arg::new("recipe_id")
                        .short('r')
                        .long("recipe-id")
                        .help("New recipe ID"),
                )
                .arg(
                    Arg::new("title")
                        .short('t')
                        .long("title")
                        .help("New event title"),
                )
                .arg(
                    Arg::new("label_id")
                        .short('l')
                        .long("label-id")
                        .help("New meal label ID"),
                ),
        )
        .subcommand(
            Command::new("delete")
                .about("Delete a meal plan event")
                .arg(Arg::new("calendar_id").required(true).help("Calendar ID"))
                .arg(Arg::new("event_id").required(true).help("Event ID")),
        )
}

pub async fn exec_command(matches: &ArgMatches) -> Result<(), CliError> {
    let tokens = read_tokens()?;
    let client = AnyListClient::from_tokens(tokens)?;

    match matches.subcommand() {
        Some(("list", sub_matches)) => {
            let start_date = sub_matches.get_one::<String>("start_date").unwrap();
            let end_date = sub_matches.get_one::<String>("end_date").unwrap();

            let events = client.get_meal_plan_events(start_date, end_date).await?;

            if events.is_empty() {
                println!("No meal plan events found in this date range.");
            } else {
                println!("Meal plan events ({} to {}):", start_date, end_date);
                println!();
                for event in events {
                    println!("Date: {}", event.date);
                    if let Some(title) = &event.title {
                        println!("  Title: {}", title);
                    }
                    if let Some(recipe_id) = &event.recipe_id {
                        println!("  Recipe ID: {}", recipe_id);
                    }
                    if let Some(label_id) = &event.label_id {
                        println!("  Label ID: {}", label_id);
                    }
                    if let Some(details) = &event.details {
                        println!("  Details: {}", details);
                    }
                    println!("  Event ID: {}", event.id);
                    println!();
                }
            }
        }
        Some(("add", sub_matches)) => {
            let calendar_id = sub_matches.get_one::<String>("calendar_id").unwrap();
            let date = sub_matches.get_one::<String>("date").unwrap();
            let recipe_id = sub_matches.get_one::<String>("recipe_id").map(|s| s.as_str());
            let title = sub_matches.get_one::<String>("title").map(|s| s.as_str());
            let label_id = sub_matches.get_one::<String>("label_id").map(|s| s.as_str());

            let event = client
                .create_meal_plan_event(calendar_id, date, recipe_id, title, label_id)
                .await?;

            println!("Created meal plan event for {}", event.date);
            if let Some(t) = &event.title {
                println!("  Title: {}", t);
            }
            if let Some(r) = &event.recipe_id {
                println!("  Recipe ID: {}", r);
            }
        }
        Some(("update", sub_matches)) => {
            let calendar_id = sub_matches.get_one::<String>("calendar_id").unwrap();
            let event_id = sub_matches.get_one::<String>("event_id").unwrap();
            let date = sub_matches.get_one::<String>("date").unwrap();
            let recipe_id = sub_matches.get_one::<String>("recipe_id").map(|s| s.as_str());
            let title = sub_matches.get_one::<String>("title").map(|s| s.as_str());
            let label_id = sub_matches.get_one::<String>("label_id").map(|s| s.as_str());

            client
                .update_meal_plan_event(calendar_id, event_id, date, recipe_id, title, label_id)
                .await?;

            println!("Updated meal plan event for {}", date);
        }
        Some(("delete", sub_matches)) => {
            let calendar_id = sub_matches.get_one::<String>("calendar_id").unwrap();
            let event_id = sub_matches.get_one::<String>("event_id").unwrap();

            client.delete_meal_plan_event(calendar_id, event_id).await?;

            println!("Deleted meal plan event");
        }
        _ => {
            println!("Unknown meal-plan command. Use --help for usage.");
        }
    }

    Ok(())
}
