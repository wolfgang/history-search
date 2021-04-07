use std::{env, panic};
use std::cell::RefCell;
use std::io::{stdout, Stdout};
use std::panic::PanicInfo;
use std::rc::Rc;

use crossterm::event::{Event, read};
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
            let stdout_ref = Rc::new(RefCell::new(stdout()));
            let (display_width, _) = size()?;
            let display_height = 11;
            let mut item_list = ItemListView::new(display_width, display_height, stdout_ref);
            let mut item_list_model = ItemListModel::new(items);
            let mut controller = ItemListController::new(&mut item_list, &mut item_list_model);
            return main_loop(&mut controller);
        }
        Err(e) => {
            println!("Error: Reading history failed: {}", e);
            Err(e.into())
        }
    }
}

fn main_loop(controller: &mut ItemListController<Stdout>) -> crossterm::Result<()> {
    enable_raw_mode()?;
    controller.init()?;
    loop {
        if let Event::Key(key_event) = read()? {
            match controller.handle_key_event(key_event) {
                Ok(false) => { break; }
                Err(e) => {
                    disable_raw_mode()?;
                    return Err(e.into());
                }
                _ => {}
            }
        }
    }
    disable_raw_mode()
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
