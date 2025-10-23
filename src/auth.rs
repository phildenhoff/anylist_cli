use anylist_rs::{AnyListClient, SavedTokens};
use std::fs;
use std::path::{Path, PathBuf};

use crate::error::CliError;

fn get_or_create_config_dir() -> Result<PathBuf, CliError> {
    let config_dir = dirs::config_dir()
        .ok_or(CliError::ConfigDirNotFound)?
        .join("anylist_rs");

    if !Path::new(&config_dir).exists() {
        fs::create_dir_all(&config_dir)
            .map_err(CliError::ConfigDirCreationFailed)?;
    }

    Ok(config_dir)
}

pub fn read_tokens() -> Result<SavedTokens, CliError> {
    let config_dir = get_or_create_config_dir()?;
    let config_file = config_dir.join("config.json");

    if !Path::new(&config_file).exists() {
        return Err(CliError::ConfigFileNotFound);
    }

    let config_contents = fs::read_to_string(&config_file)?;
    let config: serde_json::Value = serde_json::from_str(&config_contents)?;

    let access_token = config["access_token"]
        .as_str()
        .ok_or_else(|| CliError::ConfigFileInvalid("access_token".to_string()))?
        .to_string();

    let refresh_token = config["refresh_token"]
        .as_str()
        .ok_or_else(|| CliError::ConfigFileInvalid("refresh_token".to_string()))?
        .to_string();

    let user_id = config["user_id"]
        .as_str()
        .ok_or_else(|| CliError::ConfigFileInvalid("user_id".to_string()))?
        .to_string();

    let is_premium_user = config["is_premium"]
        .as_str()
        .ok_or_else(|| CliError::ConfigFileInvalid("is_premium".to_string()))?
        .parse::<bool>()
        .map_err(|_| CliError::ConfigFileInvalid("is_premium (invalid boolean)".to_string()))?;

    Ok(SavedTokens {
        access_token,
        refresh_token,
        user_id,
        is_premium_user,
    })
}

pub fn save_credentials(client: AnyListClient) -> Result<(), CliError> {
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
