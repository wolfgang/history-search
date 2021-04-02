
use std::env;
use console::{Term};
use hs::item_list::ItemList;
use hs::item_list_controller::ItemListController;
use hs::item_storage::ItemStorage;

fn main() -> std::io::Result<()> {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    if !args.is_empty() && args[0] == "-h" { return display_help() }

    let item_storage = ItemStorage::new();
    let items = item_storage.read_items();
    let term = Term::stdout();
    let mut item_list = ItemList::new(&term, &items);
    return ItemListController::new(&term, &mut item_list).run();
}

fn display_help() -> std::io::Result<()> {
    println!("\nUsage: hs");
    println!("\nDisplays a searchable list of the shell history:");
    println!("  Enter to execute the selected command,");
    println!("  Arrow up/down to change selection");
    println!("  Escape to cancel");

    return Ok(())

}
