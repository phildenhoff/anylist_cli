use std::fmt;

#[derive(Debug)]
pub enum CliError {
    ConfigDirNotFound,
    ConfigDirCreationFailed(std::io::Error),
    ConfigFileNotFound,
    ConfigFileInvalid(String),
    LoginFailed(String),
    PromptCancelled,
    ListNotFound(String),
    AnyListError(anylist_rs::Error),
    IoError(std::io::Error),
    JsonError(serde_json::Error),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CliError::ConfigDirNotFound => {
                write!(f, "Could not determine config directory. Please check your system configuration.")
            }
            CliError::ConfigDirCreationFailed(err) => {
                write!(f, "Failed to create config directory: {}", err)
            }
            CliError::ConfigFileNotFound => {
                write!(f, "Config file not found. Please run 'anylist login' first.")
            }
            CliError::ConfigFileInvalid(field) => {
                write!(f, "Config file is invalid: {} not found. Please run 'anylist login' again.", field)
            }
            CliError::LoginFailed(msg) => {
                write!(f, "Login failed: {}", msg)
            }
            CliError::PromptCancelled => {
                write!(f, "Operation cancelled by user")
            }
            CliError::ListNotFound(name) => {
                write!(f, "List '{}' not found", name)
            }
            CliError::AnyListError(err) => {
                write!(f, "AnyList API error: {}", err)
            }
            CliError::IoError(err) => {
                write!(f, "IO error: {}", err)
            }
            CliError::JsonError(err) => {
                write!(f, "JSON parsing error: {}", err)
            }
        }
    }
}

impl std::error::Error for CliError {}

impl From<anylist_rs::Error> for CliError {
    fn from(err: anylist_rs::Error) -> Self {
        CliError::AnyListError(err)
    }
}

impl From<std::io::Error> for CliError {
    fn from(err: std::io::Error) -> Self {
        CliError::IoError(err)
    }
}

impl From<serde_json::Error> for CliError {
    fn from(err: serde_json::Error) -> Self {
        CliError::JsonError(err)
    }
}

impl From<inquire::InquireError> for CliError {
    fn from(err: inquire::InquireError) -> Self {
        match err {
            inquire::InquireError::OperationCanceled | inquire::InquireError::OperationInterrupted => {
                CliError::PromptCancelled
            }
            _ => CliError::LoginFailed(err.to_string()),
        }
    }
}
