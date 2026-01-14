use anylist_rs::{AnyListClient, Ingredient, Recipe};
use clap::{Arg, ArgMatches, Command};

use crate::auth::read_tokens;
use crate::error::CliError;

fn display_recipe_list(recipes: Vec<Recipe>) {
    if recipes.is_empty() {
        println!("No recipes found.");
        return;
    }

    println!("\nYour Recipes:");
    println!("{}", "=".repeat(13));
    println!();

    // Sort recipes by name
    let mut sorted = recipes;
    sorted.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    for recipe in sorted {
        let ingredient_count = recipe.ingredients.len();
        let step_count = recipe.preparation_steps.len();
        print!("  \x1B[1m{}\x1B[0m", recipe.name);

        if ingredient_count > 0 || step_count > 0 {
            print!(" ({} ingredients, {} steps)", ingredient_count, step_count);
        }

        if let Some(rating) = recipe.rating {
            let stars = "★".repeat(rating as usize);
            print!(" {}", stars);
        }

        println!();
    }
    println!();
}

fn display_ingredient(ingredient: &Ingredient) {
    print!("    • {}", ingredient.name);
    if let Some(qty) = &ingredient.quantity {
        print!(": {}", qty);
    }
    if let Some(note) = &ingredient.note {
        print!(" ({})", note);
    }
    println!();
}

fn display_recipe_detail(recipe: &Recipe) {
    println!("\n\x1B[1m{}\x1B[0m", recipe.name);
    println!("{}", "=".repeat(recipe.name.len()));
    println!();

    // Display ID
    println!("ID: {}", recipe.id);

    // Display rating
    if let Some(rating) = recipe.rating {
        let stars = "★".repeat(rating as usize);
        println!("Rating: {}", stars);
    }

    // Display source
    if let Some(source_name) = &recipe.source_name {
        print!("Source: {}", source_name);
        if let Some(source_url) = &recipe.source_url {
            print!(" ({})", source_url);
        }
        println!();
    }

    // Display servings
    if let Some(servings) = &recipe.servings {
        println!("Servings: {}", servings);
    }

    // Display times (convert from seconds to minutes)
    if let Some(prep_time) = recipe.prep_time {
        println!("Prep Time: {} minutes", prep_time / 60);
    }
    if let Some(cook_time) = recipe.cook_time {
        println!("Cook Time: {} minutes", cook_time / 60);
    }

    // Display note
    if let Some(note) = &recipe.note {
        println!("\nNote: {}", note);
    }

    // Display ingredients
    if !recipe.ingredients.is_empty() {
        println!("\n\x1B[1mIngredients:\x1B[0m");
        for ingredient in &recipe.ingredients {
            display_ingredient(ingredient);
        }
    }

    // Display preparation steps
    if !recipe.preparation_steps.is_empty() {
        println!("\n\x1B[1mPreparation:\x1B[0m");
        for (i, step) in recipe.preparation_steps.iter().enumerate() {
            println!("  {}. {}", i + 1, step);
        }
    }

    println!();
}

pub fn command() -> Command {
    Command::new("recipe")
        .about("View and manage your AnyList recipes")
        .long_about(
            "View and manage your AnyList recipes.\n\n\
             By default, this command shows all your recipes.\n\
             Use subcommands to view recipe details.",
        )
        .subcommand(
            Command::new("list")
                .about("List all recipes")
                .long_about("Display a list of all your recipes with ingredient and step counts"),
        )
        .subcommand(
            Command::new("get")
                .about("Display details for a specific recipe")
                .arg(
                    Arg::new("name")
                        .help("Name or ID of the recipe to display")
                        .required(true)
                        .value_name("RECIPE_NAME_OR_ID"),
                ),
        )
}

pub async fn exec_command(matches: &ArgMatches) -> Result<(), CliError> {
    let tokens = read_tokens()?;
    let client = AnyListClient::from_tokens(tokens)?;

    match matches.subcommand() {
        Some(("list", _)) => {
            let recipes = client.get_recipes().await?;
            display_recipe_list(recipes);
        }
        Some(("get", sub_matches)) => {
            let identifier = sub_matches
                .get_one::<String>("name")
                .expect("required argument");

            // Try to get by name first, fall back to ID
            let recipe = match client.get_recipe_by_name(identifier).await {
                Ok(r) => r,
                Err(_) => client.get_recipe_by_id(identifier).await?,
            };

            display_recipe_detail(&recipe);
        }
        _ => {
            // Default: show all recipes
            let recipes = client.get_recipes().await?;
            display_recipe_list(recipes);
        }
    }

    Ok(())
}
