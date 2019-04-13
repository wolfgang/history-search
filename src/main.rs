
mod item_list;
mod item_list_controller;



use std::fs::File;
use std::io::{BufRead, BufReader};
use console::{Term};
use item_list::ItemList;
use item_list_controller::ItemListController;

fn main() -> std::io::Result<()> {
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
