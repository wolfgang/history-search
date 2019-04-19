
use std::io::prelude::*;
use std::io::{BufRead, BufReader};
use std::fs::{OpenOptions, DirBuilder, File};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use console::{style};

use std::env;


pub fn init() -> ItemStorage {
    ItemStorage::new(&get_default_home_dir())
}

pub struct ItemStorage {
    item_file: String
}

impl ItemStorage {
    pub fn new(home_dir: &str) -> ItemStorage {
        let home_dir_path = Path::new(home_dir);

        if !home_dir_path.exists() {
            DirBuilder::new()
                .recursive(false)
                .create(home_dir).expect("Failed to create home dir");
        }

        if !home_dir_path.join("items.txt").exists() {
            OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(home_dir_path.join("items.txt"))
                .expect("Create file failed");            
        }

        ItemStorage {
            item_file: format!("{}/items.txt", home_dir)
        }
    }

    pub fn read_items(&self) -> Vec<String> {
        let mut lines = self.read_lines_from_item_file();

        lines.sort_by(|line_a: &String, line_b: &String| {
            let timestamp_a = get_timestamp(line_a);
            let timestamp_b = get_timestamp(line_b);

            timestamp_b.partial_cmp(&timestamp_a).unwrap()
        });

        lines.into_iter().map(|line| { parse_command(&line) }).collect()
    }

    pub fn add_item(&self, args: &mut Vec<String>) -> std::io::Result<()> {
        let mut prefix = String::from("");
        if args[0] == "-d" {
            args.remove(0);
            if args.is_empty() {
                panic!("Error: Must add command if specifying -d");
            }

            prefix = format!("[{}]", get_cwd());    
        }

        let entry_without_timestamp = format!("{}{}", prefix, args.join(" "));

        if self.read_items().contains(&entry_without_timestamp) {
            println!("{}", style("Not adding duplicate entry").red());
            return Ok(());
        }

        let entry = format!("{} {}", get_now_timestamp(), entry_without_timestamp);
        let mut file = OpenOptions::new().append(true).open(&self.item_file)?;
        write!(file, "{}\n", entry)?;
        println!("Added entry: {}", style(entry).green());
        return Ok(());
    }

    pub fn replace_timestamp(&self, cmd: &str) {
        let lines = self.read_lines_from_item_file();

        let new_lines: Vec<String> = lines.into_iter().map(|line| {
            let line_cmd = parse_command(&line);
            if line_cmd == cmd { format!("{} {}", get_now_timestamp(), cmd) }
            else { line.to_string() }
        }).collect();


        let mut file = OpenOptions::new().write(true).open(&self.item_file).unwrap();
        write!(file, "{}", new_lines.join("\n")).unwrap();
    }

    fn read_lines_from_item_file(&self) -> Vec<String> {
        let file = File::open(&self.item_file).unwrap();
        let reader = BufReader::new(file);

        reader.lines()
            .map(|line| { line.unwrap().to_string() })
            .collect()
    }

}


fn get_now_timestamp() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}

fn get_cwd() -> String {
    env::current_dir().unwrap().as_path().to_str().unwrap().to_string()
}

fn parse_command(line: &str) -> String {
    let after_ts = line.find(' ').unwrap_or(0);
    let s = &line[after_ts + 1..];
    s.to_string()
}

fn get_timestamp(line: &str) -> u64 {
    let after_ts = line.find(' ').unwrap_or(0);
    let s = &line[..after_ts];
    s.to_string().parse().unwrap()
}


fn get_default_home_dir() -> String {
    let home = env::var("HOME").unwrap();
    format!("{}/.rp", home)
}

