use crate::item_list::ItemList;

use std::process::Command;
use std::env;

use console::{Key, Term};

pub struct ItemListController<'a> {
    term: &'a Term,
    item_list: &'a mut ItemList<'a>
}

impl<'a> ItemListController<'a> {
    pub fn new(
        term: &'a Term, 
        item_list: &'a mut ItemList<'a>) -> ItemListController<'a> {

        ItemListController { term: term, item_list: item_list }
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        self.item_list.render()?;
        self.item_list.init_cursor()?;
        loop {
            let key = self.term.read_key().unwrap();

            match key {
                Key::Enter => { return self.execute_selection() }
                Key::Escape => { return self.item_list.clear() }
                Key::ArrowUp => { self.item_list.change_selection(-1)? }
                Key::ArrowDown => { self.item_list.change_selection(1)? }
                Key::Char(ch) => { self.item_list.on_character_entered(ch)? }
                _ => {}
            }
        }
    }

    fn execute_selection(&mut self) -> std::io::Result<()> {
        self.item_list.clear()?;
        let item = self.item_list.selected_item();
        let parts : Vec<&str> = item.split("]").collect();

        let mut cmd = &parts[0];
        let mut cd = ".";
        if parts.len() == 2 {
            cd = &parts[0][1..];
            cmd = &parts[1]
        }


        let shell = env::var_os("SHELL").unwrap();
        Command::new(shell)
                .arg("-c")
                .arg(cmd)
                .current_dir(cd)
                .status()
                .expect(&format!("Failed to execute: {}", cmd ));
        Ok(())
    }
}