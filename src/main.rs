
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

    // let result = item_storage::replace_timestamp("echo HELLO3");
    // println!("{:?}", result);
    // return Ok(());


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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::prelude::*;

    #[test]
    fn term_can_be_initialized() {
        let mut term = Term::stdout();
        term.write_line("HELLO").unwrap();
        write_to_writer(&mut term, b"HELLO AGAIN").unwrap();
    }

    fn write_to_writer<W: Write>(writer: &mut W, buf: &[u8]) -> std::io::Result<()> {
        writer.write(buf)?;
        Ok(())

    }
}


