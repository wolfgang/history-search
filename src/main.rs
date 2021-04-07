use std::{env, panic};
use std::io::stdout;
use std::panic::PanicInfo;

use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size};

use hs::item_list_controller::ItemListController;
use hs::item_list_model::ItemListModel;
use hs::item_list_view::ItemListView;
use hs::item_storage::ItemStorage;

fn main() -> crossterm::Result<()> {
    configure_panic_hook();

    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    if !args.is_empty() && args[0] == "-h" { return display_help(); }

    let item_storage = ItemStorage::new();
    match item_storage.read_items() {
        Ok(items) => {
            let mut stdout = stdout();
            let (display_width, _) = size()?;
            let display_height = 11;
            let mut item_list = ItemListView::new(display_width, display_height, &mut stdout);
            let mut item_list_model = ItemListModel::new(items);
            enable_raw_mode()?;
            ItemListController::new(&mut item_list, &mut item_list_model).run()?;
            return disable_raw_mode();
        }
        Err(e) => {
            println!("Error: Reading history failed: {}", e);
            Err(e.into())
        }
    }
}

fn configure_panic_hook() {
    let panic_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info: &PanicInfo| {
        disable_raw_mode().unwrap();
        panic_hook(panic_info);
    }));
}

fn display_help() -> crossterm::Result<()> {
    println!("\nUsage: hs");
    println!("\nDisplays a searchable list of the shell history:");
    println!("  Enter to execute the selected command,");
    println!("  Arrow up/down to change selection");
    println!("  Escape to cancel");
    return Ok(());
}
