use std::env;

use walkdir::WalkDir;

fn main() {
    let home = env::var("HOME").unwrap();
    let root_path = format!("{}/.config", home);
    let allowed_items = ["helix", "starship.toml"];
    let ignored_items = [".DS_Store"];
    let mut files: Vec<String> = vec![];
    let walker = WalkDir::new(root_path).min_depth(1).into_iter();
    for entry in walker.filter_entry(|e| {
        let path = e.path().to_str().unwrap();
        let mut is_allowed = false;
        for item_name in ignored_items {
            if path.contains(item_name) {
                return is_allowed;
            }
        }

        for item_name in allowed_items {
            if path.contains(item_name) {
                is_allowed = true;
                break;
            };
        }

        is_allowed
    }) {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            files.push(entry.path().to_str().unwrap().to_string());
        }
    }

    for file in files {
        println!("{}", file);
    }
}
