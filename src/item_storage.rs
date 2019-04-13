
use std::io::prelude::*;
use std::process;
use std::io::{BufRead, BufReader};
use std::fs::{OpenOptions, DirBuilder, File};
use std::path::Path;

use std::env;

pub fn add_item(args: &mut Vec<String>) -> std::io::Result<()> {
    let mut prefix = String::from("");
    if args[0] == "-d" {
        args.remove(0);
        if args.is_empty() {
            println!("Error: Must add command if specifying -d");
            process::exit(1);
        }

        let cwd = env::current_dir().unwrap().as_path().to_str().unwrap().to_string();
        prefix = format!("[{}]", cwd);    
    }

    let mut file = OpenOptions::new().append(true).open(get_item_file())?;
    let entry = args.join(" ");
    write!(file, "{}{}\n", prefix, entry)?;
    println!("Added entry: {}{}", prefix, entry);
    return Ok(());
}

pub fn read_items() -> Vec<String> {
    let file = File::open(get_item_file()).unwrap();
    let reader = BufReader::new(file);

    let mut items = Vec::new();

    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        items.push(line);
    }

    items
}

pub fn init() {
    if !Path::new(&get_home_dir()).exists() {
        DirBuilder::new()
            .recursive(false)
            .create(get_home_dir()).expect("Failed to create home dir");
    
        File::create(get_item_file()).expect("Create file failed");
    }
}

fn get_item_file() -> String {
    format!("{}/items.txt", get_home_dir())
}

fn get_home_dir() -> String {
    let home = env::var("HOME").unwrap();
    format!("{}/.rp", home)
}

