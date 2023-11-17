use std::fs;
use std::path::{Path, PathBuf};
use dirs;

fn get_or_create_config_dir() -> Result<PathBuf, Box<dyn std::error::Error>>{
    let config_dir = dirs::config_dir().unwrap().join("anylist_rs");
    if !Path::new(&config_dir).exists() {
        fs::create_dir(&config_dir).unwrap();
    }

    Ok(config_dir)
}

pub fn get_signed_user_id() -> Result<String, Box<dyn std::error::Error>> {
    let config_dir = get_or_create_config_dir()?;
    let config_file = config_dir.join("config.json");

    if !Path::new(&config_file).exists() {
        return Err("Config file not found".into());
    }

    let config_contents = fs::read_to_string(&config_file)?;
    let config: serde_json::Value = serde_json::from_str(&config_contents)?;

    match config["signed_user_id"].as_str() {
        Some(id) => Ok(id.to_string()),
        None => Err("signed_user_id not found in config file".into()),
    }
}


pub fn save_credentials(signed_user_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let config_dir = get_or_create_config_dir()?;
    let config_file = config_dir.join("config.json");

    let mut config = if Path::new(&config_file).exists() {
        let config_contents = fs::read_to_string(&config_file)?;
        serde_json::from_str(&config_contents)?
    } else {
        serde_json::Value::Object(serde_json::Map::new())
    };

    config["signed_user_id"] = serde_json::Value::String(signed_user_id.to_string());

    let config_contents = serde_json::to_string(&config)?;
    fs::write(&config_file, config_contents)?;

    Ok(())
}

