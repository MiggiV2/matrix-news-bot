use serde::Deserialize;
use std::fs;
use toml::de::Error;

#[derive(Deserialize)]
pub struct NewBotConfig {
    pub matrix_homerserver: String,
    pub matrix_username: String,
    pub matrix_password: String,
    pub matrix_room_id: String,
    pub news_time: String,
    pub update_frequency: String,
    pub bot_name: String,
}

pub fn parse_config(config_path: Option<String>) -> NewBotConfig {
    let config_path = config_path.unwrap_or_else(|| {
        "./config.toml".to_string()
    });

    let contents = fs::read_to_string(&config_path);
    if let Err(e) = &contents {
        eprintln!("Please provide the config file '{}' -> {}", config_path, e);
        std::process::exit(1);
    }
    let config: Result<NewBotConfig, Error> = toml::from_str(contents.unwrap().as_str());
    if let Err(e) = &config {
        eprintln!("Please your config file is invalid! -> {}", e);
        std::process::exit(1);
    }
    config.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_config() {
        let config = parse_config(None);
        assert_eq!(config.matrix_room_id, String::from("room_id"));
    }
}