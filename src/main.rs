use serde::Deserialize;
use std::{env, fs};
use walkdir::WalkDir;

fn main() {
    let home = env::var("HOME").unwrap();
    let config_path = format!("{}/.config", home);
    let config = get_config(&config_path);
    println!("{:?}", config);
    // let allowed_items = vec!["helix", "starship.toml"];
    // let ignored_items = vec![".DS_Store", "runtime"];
    // let file_paths = get_file_paths(&root_path, allowed_items, ignored_items);
    // for path in file_paths {
    //     println!("{}", path);
    // }
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub client_name: String,
    pub root: String,
    pub file: Vec<File>,
    pub folder: Vec<Folder>,
}

#[derive(Debug, Deserialize)]
pub struct File {
    pub name: String,
    pub last_synced: String,
}

#[derive(Debug, Deserialize)]
pub struct Folder {
    pub name: String,
    pub ignored_items: Vec<String>,
    pub last_synced: String,
}

fn get_config(config_path: &str) -> Config {
    let toml_string = fs::read_to_string(format!("{}/{}", config_path, "sink.toml"))
        .expect("Failed to find config file");

    toml::from_str(&toml_string).expect("Failed to parse config file")
}

fn get_file_paths(
    root_path: &str,
    allowed_items: Vec<&str>,
    ignored_items: Vec<&str>,
) -> Vec<String> {
    let mut file_paths: Vec<String> = vec![];
    let walker = WalkDir::new(root_path).min_depth(1).into_iter();
    for entry in walker.filter_entry(|e| {
        let path = e.path().to_str().unwrap();
        let mut is_allowed = false;

        for item_name in ignored_items.as_slice() {
            if path.contains(item_name) {
                return is_allowed;
            }
        }

        for item_name in allowed_items.as_slice() {
            if path.contains(item_name) {
                is_allowed = true;
                break;
            };
        }

        is_allowed
    }) {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            file_paths.push(entry.path().to_str().unwrap().to_string());
        }
    }

    file_paths
}
