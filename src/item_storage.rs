use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub struct ItemStorage {
    item_file: String
}

impl ItemStorage {
    pub fn new() -> ItemStorage {
        let home_value = env::var("HOME").unwrap();
        let home_dir = Path::new(home_value.as_str());
        let bash_history = String::from(home_dir.join(".bash_history").to_str().unwrap());
        let histfile_value = env::var("HISTFILE").unwrap_or(bash_history);
        ItemStorage { item_file: histfile_value }
    }

    pub fn read_items(&self) -> std::io::Result<Vec<String>> {
        let file = File::open(&self.item_file)?;
        let reader = BufReader::new(file);

        let mut lines: Vec<String> = reader
            .lines()
            .map(|line| { line.unwrap().to_string() })
            .collect();

        lines.reverse();
        Ok(lines)
    }
}
