
use std::io::prelude::*;
use std::process;
use std::io::{BufRead, BufReader};
use std::fs::{OpenOptions, DirBuilder, File};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use regex::Regex;


use console::{style};

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

    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    let entry = format!("{} {}{}", timestamp.as_secs(), prefix, args.join(" "));

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

    lines.into_iter().map(|line| {
        let after_ts = line.find(' ').unwrap_or(0);
        let line_cmd = &line[after_ts + 1..];
        if line_cmd == cmd {
            let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
            format!("{} {}", timestamp.as_secs(), cmd)
        }
        else {
            line_cmd.to_string()
        }
    }).collect()

}

fn get_cmd(line: &str) -> String {
    let after_ts = line.find(' ').unwrap_or(0);
    let s = &line[after_ts + 1..];
    s.to_string()
}

fn get_timestamp(line: &str) -> u64 {
    let re = Regex::new(r"^(\d+)\s+(.*)").unwrap();
    let caps = re.captures(line).unwrap();
    caps.get(1).unwrap().as_str().parse::<u64>().unwrap()
}

pub fn init() {
    if !Path::new(&get_home_dir()).exists() {
        DirBuilder::new()
            .recursive(false)
            .create(get_home_dir()).expect("Failed to create home dir");
    }

    if !Path::new(&get_item_file()).exists() {
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

