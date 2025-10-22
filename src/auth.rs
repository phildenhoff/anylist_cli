use anylist_rs::login::LoginResult;
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

pub fn read_login() -> Result<LoginResult, Box<dyn std::error::Error>> {
    let config_dir = get_or_create_config_dir()?;
    let config_file = config_dir.join("config.json");

    if !Path::new(&config_file).exists() {
        return Err("Config file not found".into());
    }

    let config_contents = fs::read_to_string(&config_file)?;
    let config: serde_json::Value = serde_json::from_str(&config_contents)?;

    let signed_user_id = match config["signed_user_id"].as_str() {
        Some(id) => id.to_string(),
        None => return Err("signed_user_id not found in config file".into()),
    };

    let user_id = match config["user_id"].as_str() {
        Some(id) => id.to_string(),
        None => return Err("user_id not found in config file".into()),
    };

    let is_premium_user = match config["is_premium"].as_str() {
        Some(value) => value.parse::<bool>()?,
        None => return Err("is_premium not found in config file".into()),
    };

    Ok(LoginResult {
        credential: signed_user_id,
        user_id,
        is_premium_user,
    })
}

pub fn save_login(login_result: LoginResult) -> Result<(), Box<dyn std::error::Error>> {
    let config_dir = get_or_create_config_dir()?;
    let config_file = config_dir.join("config.json");

    let mut config = if Path::new(&config_file).exists() {
        let config_contents = fs::read_to_string(&config_file)?;
        serde_json::from_str(&config_contents)?
    } else {
        serde_json::Value::Object(serde_json::Map::new())
    };

    config["signed_user_id"] = serde_json::Value::String(login_result.credential.to_string());
    config["user_id"] = serde_json::Value::String(login_result.user_id.to_string());
    config["is_premium"] = serde_json::Value::String(login_result.is_premium_user.to_string());

    let config_contents = serde_json::to_string(&config)?;
    fs::write(&config_file, config_contents)?;

    Ok(())
}
