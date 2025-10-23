use anylist_rs::{AnyListClient, SavedTokens};
use dirs;
use std::fs;
use std::path::{Path, PathBuf};

fn get_or_create_config_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let config_dir = dirs::config_dir().unwrap().join("anylist_rs");
    if !Path::new(&config_dir).exists() {
        fs::create_dir(&config_dir).unwrap();
    }

    Ok(config_dir)
}

pub fn read_tokens() -> Result<SavedTokens, Box<dyn std::error::Error>> {
    let config_dir = get_or_create_config_dir()?;
    let config_file = config_dir.join("config.json");

    if !Path::new(&config_file).exists() {
        return Err("Config file not found".into());
    }

    let config_contents = fs::read_to_string(&config_file)?;
    let config: serde_json::Value = serde_json::from_str(&config_contents)?;

    let access_token = match config["access_token"].as_str() {
        Some(id) => id.to_string(),
        None => return Err("access_token not found in config file".into()),
    };

    let refresh_token = match config["refresh_token"].as_str() {
        Some(id) => id.to_string(),
        None => return Err("refresh_token not found in config file".into()),
    };

    let user_id = match config["user_id"].as_str() {
        Some(id) => id.to_string(),
        None => return Err("user_id not found in config file".into()),
    };

    let is_premium_user = match config["is_premium"].as_str() {
        Some(value) => value.parse::<bool>()?,
        None => return Err("is_premium not found in config file".into()),
    };

    Ok(SavedTokens {
        access_token,
        refresh_token,
        user_id,
        is_premium_user,
    })
}

pub fn save_credentials(client: AnyListClient) -> Result<(), Box<dyn std::error::Error>> {
    let config_dir = get_or_create_config_dir()?;
    let config_file = config_dir.join("config.json");

    let mut config = if Path::new(&config_file).exists() {
        let config_contents = fs::read_to_string(&config_file)?;
        serde_json::from_str(&config_contents)?
    } else {
        serde_json::Value::Object(serde_json::Map::new())
    };

    let tokens = client.export_tokens()?;

    config["access_token"] = serde_json::Value::String(tokens.access_token.to_string());
    config["refresh_token"] = serde_json::Value::String(tokens.refresh_token.to_string());
    config["user_id"] = serde_json::Value::String(tokens.user_id.to_string());
    config["is_premium"] = serde_json::Value::String(tokens.is_premium_user.to_string());

    let config_contents = serde_json::to_string(&config)?;
    fs::write(&config_file, config_contents)?;

    Ok(())
}
