use std::fs;

use serde_derive::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct ServePath {
    pub web_path: String,
    pub file_path: String,
    pub index_file: Option<String>,
    pub show_index: Option<bool>,
    pub host: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ConfigFile {
    pub bind: String,
    pub workers: Option<usize>,
    pub serve: Vec<ServePath>,
}

pub fn load_config_file(file_name: String) -> Option<ConfigFile> {
    let contents = match fs::read_to_string(&file_name) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Could not read file `{}`", file_name);
            return None;
        }
    };

    let config: ConfigFile = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(_) => {
            eprintln!("Could not parse config file `{}`", file_name);
            return None;
        }
    };

    Some(config)
}
