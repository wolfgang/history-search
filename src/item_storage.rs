
use std::io::prelude::*;
use std::process;
use std::io::{BufRead, BufReader};
use std::fs::{OpenOptions, DirBuilder, File};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use console::{style};

use std::env;

pub struct ItemStorage {

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

        ItemStorage {}
    }
}



pub fn call_me() -> u32 {
    1234
}

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

    let entry = format!("{} {}{}", get_now_timestamp(), prefix, args.join(" "));

    if read_items().contains(&entry) {
        println!("{}", style("Not adding duplicate entry").red());
        return Ok(());
    }

    let mut file = OpenOptions::new().append(true).open(get_item_file())?;
    write!(file, "{}\n", entry)?;
    println!("Added entry: {}", style(entry).green());
    return Ok(());
}

pub fn read_items() -> Vec<String> {
    let file = File::open(get_item_file()).unwrap();
    let reader = BufReader::new(file);

    let mut lines = Vec::new();

    for (_, line) in reader.lines().enumerate() { 
        let line = line.unwrap();
        lines.push(line.to_string());
    }

    lines.sort_by(|line_a: &String, line_b: &String| {
        let timestamp_a = get_timestamp(line_a);
        let timestamp_b = get_timestamp(line_b);

        timestamp_b.partial_cmp(&timestamp_a).unwrap()
    });

    lines.into_iter().map(|line| { get_cmd(&line) }).collect()

}

pub fn replace_timestamp(cmd: &str) -> Vec<String> {
    let file = File::open(get_item_file()).unwrap();
    let reader = BufReader::new(file);

    let mut lines = Vec::new();

    for (_, line) in reader.lines().enumerate() { 
        let line = line.unwrap();
        lines.push(line.to_string());
    }

    let new_lines: Vec<String> = lines.into_iter().map(|line| {
        let line_cmd = get_cmd(&line);
        if line_cmd == cmd { format!("{} {}", get_now_timestamp(), cmd) }
        else { line.to_string() }
    }).collect();


    let mut file = OpenOptions::new().write(true).open(get_item_file()).unwrap();
    write!(file, "{}", new_lines.join("\n")).unwrap();

    new_lines

}

fn get_now_timestamp() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}

fn get_cmd(line: &str) -> String {
    let after_ts = line.find(' ').unwrap_or(0);
    let s = &line[after_ts + 1..];
    s.to_string()
}

fn get_timestamp(line: &str) -> u64 {
    let after_ts = line.find(' ').unwrap_or(0);
    let s = &line[..after_ts];
    s.to_string().parse().unwrap()
}

pub fn init() {
    ItemStorage::new(&get_home_dir());
}

fn get_item_file() -> String {
    format!("{}/items.txt", get_home_dir())
}

fn get_home_dir() -> String {
    let home = env::var("HOME").unwrap();
    format!("{}/.rp", home)
}

