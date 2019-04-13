use crate::item_list::ItemList;

use std::process::Command;
use std::env;

use console::{Key, Term};

pub struct ItemListController<'a> {
    term: &'a Term,
    item_list_ref: &'a mut ItemList<'a>
}

impl<'a> ItemListController<'a> {
    pub fn new(
        term: &'a Term, 
        item_list_ref: &'a mut ItemList<'a>) -> ItemListController<'a> {

        ItemListController { term: term, item_list_ref: item_list_ref }
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        self.item_list_ref.render()?;
        self.item_list_ref.init_cursor()?;
        loop {
            let key = self.term.read_key().unwrap();

            match key {
                Key::Enter => {
                    self.item_list_ref.clear()?;
                    let cmd = self.item_list_ref.selected_item();
                    println!("Selected: {:}", cmd);

                    let shell = env::var_os("SHELL").unwrap();
                    Command::new(shell)
                            .arg("-c")
                            .arg(cmd)
                            .status()
                            .expect(&format!("Failed to execute: {}", cmd ));
                    return Ok(());
                }
                Key::Escape => { 
                    return self.item_list_ref.clear()
                }
                Key::ArrowUp => { self.item_list_ref.change_selection(-1)?; }
                Key::ArrowDown => { self.item_list_ref.change_selection(1)?; }
                Key::Char(ch) => { self.item_list_ref.on_character_entered(ch)?; }
                _ => {}
            }
        }
    }
}