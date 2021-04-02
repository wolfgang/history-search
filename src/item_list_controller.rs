use std::env;
use std::process::Command;

use crossterm::event::{Event, KeyCode, read};
use crossterm::style::Colorize;

use crate::item_list::ItemList;

pub struct ItemListController<'a> {
    item_list: &'a mut ItemList<'a>,
}

impl<'a> ItemListController<'a> {
    pub fn new(item_list: &'a mut ItemList<'a>) -> ItemListController<'a> {
        ItemListController { item_list }
    }

    pub fn run(&mut self) -> crossterm::Result<()> {
        self.item_list.init()?;

        loop {
            if let Event::Key(key_event) = read().unwrap() {
                match key_event.code {
                    KeyCode::Enter => { return self.execute_selection(); }
                    KeyCode::Esc => { return self.item_list.clear(); }
                    KeyCode::Down => { self.item_list.change_selection(1)? }
                    KeyCode::Up => { self.item_list.change_selection(-1)? }
                    KeyCode::Backspace => { self.item_list.on_backspace()? }
                    KeyCode::Char(ch) => { self.item_list.on_character_entered(ch)? }
                    _ => {}
                }
            }
        }
    }

    fn execute_selection(&mut self) -> crossterm::Result<()> {
        self.item_list.remove()?;
        let command = self.item_list.selected_item().to_string();
        self.print_command_info(&command);
        execute_command(&command);
        self.item_list.reset_cursor()?;
        Ok(())
    }

    fn print_command_info(&self, command: &str) {
        println!("-> {}\r", &command.green());
    }
}

fn execute_command(command: &str) {
    let shell = env::var_os("SHELL").unwrap();
    Command::new(shell)
        .arg("-c")
        .arg(&command)
        .status()
        .expect(&format!("Failed to execute: {}", command));
}

