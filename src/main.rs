
mod item_list;
mod item_list_controller;

use std::io::prelude::*;
use std::process;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader};
use std::env;
use console::{Term};
use item_list::ItemList;
use item_list_controller::ItemListController;

fn main() -> std::io::Result<()> {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    if !args.is_empty() {
        let mut add_cwd = false;
        if args[0] == "-d" {
            add_cwd = true;
            args.remove(0);
            if args.is_empty() {
                println!("Error: Must add command if specifying -d");
                process::exit(1);
            }
        }

        println!("{:?}", add_cwd);

        let mut file = OpenOptions::new().append(true).open("test.txt")?;
        let entry = args.join(" ");
        write!(file, "{}\n", entry)?;
        println!("Added entry: {}", entry);
        return Ok(());
    }


    let items = read_items("test.txt");
    let term = Term::stdout();

    let mut item_list = ItemList::new(&term, &items);
    let mut item_list_controller = ItemListController::new(&term, &mut item_list);
    item_list_controller.run()
}

fn read_items(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    let mut items = Vec::new();

    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        items.push(line);
    }

    items
}
