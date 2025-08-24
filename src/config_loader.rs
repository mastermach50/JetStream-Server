use std::fs;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ServerConfig {
    pub server_name: String,

    pub addr: String,
    pub port: u16,
    pub password: String,

    pub allow_sync_clipboard: bool
}
pub trait ServerConfigExt {
    fn default() -> Self;
}

impl ServerConfigExt for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            server_name: "Mathew's Jetstream Server".to_string(),
            addr: "0.0.0.0".to_string(),
            port: 8000,
            password: "".to_string(),
            allow_sync_clipboard: false
        }
    }
}

#[cfg(target_os = "linux")]
const CONFIG_PATH: &str = "~/.config/jetstream/config.toml";

#[cfg(target_os = "windows")]
const CONFIG_PATH: &str = "~/AppData/Roaming/jetstream/config.toml";

pub fn get_config() -> ServerConfig {
    match fs::read_to_string(CONFIG_PATH) {
        Ok(config) => {
            toml::from_str(&config).unwrap()
        },
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                eprintln!("Config file not found at {CONFIG_PATH}");
                // TODO implement writing default config file
            } else {
                eprintln!("Failed to read config file: {e}");
            }
            println!("Running with default config");

            ServerConfig::default()
        }
}
}