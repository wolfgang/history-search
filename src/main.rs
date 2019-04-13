
mod item_storage;
mod item_list;
mod item_list_controller;

use std::env;
use console::{Term};
use item_list::ItemList;
use item_list_controller::ItemListController;

fn main() -> std::io::Result<()> {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    item_storage::init();

    if !args.is_empty() {
        return item_storage::add_item(&mut args);
    }

    let items = item_storage::read_items();
    let term = Term::stdout();
    let mut item_list = ItemList::new(&term, &items);
    return ItemListController::new(&term, &mut item_list).run();
}

