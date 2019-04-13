
mod item_list;

use std::env;
use std::process::Command;
use std::fs::File;
use std::io::{BufRead, BufReader};
use console::{Key, Term};
use item_list::ItemList;

fn main() -> std::io::Result<()> {
    let items = read_items("test.txt");
    let term = Term::stdout();

    let mut item_list = ItemList::new(&term, &items);
    item_list.render()?;
    item_list.init_cursor()?;

    loop {
        let key = term.read_key().unwrap();

        match key {
            Key::Enter => {
                item_list.clear()?;
                let cmd = item_list.selected_item();
                println!("Selected: {:}", cmd);

                let shell = env::var_os("SHELL").unwrap();
                Command::new(shell)
                        .arg("-c")
                        .arg(cmd)
                        .status()
                        .expect(&format!("Failed to execute: {}", cmd ));
                return Ok(());
            }
            Key::Escape => { return Ok(()); }
            Key::ArrowUp => { item_list.change_selection(-1)?; }
            Key::ArrowDown => { item_list.change_selection(1)?; }
            Key::Char(ch) => { item_list.on_character_entered(ch)?; }
            _ => {}
        }
    }
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
