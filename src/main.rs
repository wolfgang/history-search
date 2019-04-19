
use std::env;
use console::{Term};
use rp::item_storage;
use rp::item_list::ItemList;
use rp::item_list_controller::ItemListController;

fn main() -> std::io::Result<()> {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    item_storage::init();

    if !args.is_empty() {
        if args[0] == "-h" { return display_help() }
        return item_storage::add_item(&mut args);
    }

    let items = item_storage::read_items();
    let term = Term::stdout();
    let mut item_list = ItemList::new(&term, &items);
    return ItemListController::new(&term, &mut item_list).run();
}

fn display_help() -> std::io::Result<()> {
    println!("\nUsage: rp [OPTIONS] INPUT");
    println!("Store arbitrary command given in INPUT.");
    println!("\nOptions:");
    println!("  -d      store current directory along with command");
    println!("  -h      show this message");
    println!("\n");
    println!("If no arguments are given, displays a searchable list:");
    println!("  Enter to execute the selected command,");
    println!("  Arrow up/down to change selection");
    println!("  Escape to cancel");
    println!("\nCommands are stored in ~/.rp/items.txt");

    return Ok(())

}
