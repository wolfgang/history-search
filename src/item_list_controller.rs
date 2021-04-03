use std::env;
use std::process::Command;

use crossterm::event::{Event, KeyCode, read};
use crossterm::style::Colorize;
use crossterm::terminal::disable_raw_mode;

use crate::item_list::ItemList;
use crate::item_list_model::ItemListModel;

pub struct ItemListController<'a> {
    item_list: &'a mut ItemList,
    item_list_model: &'a mut ItemListModel<'a>,
}

impl<'a> ItemListController<'a> {
    pub fn new(item_list: &'a mut ItemList, item_list_model: &'a mut ItemListModel<'a>) -> ItemListController<'a> {
        ItemListController { item_list, item_list_model }
    }

    pub fn run(&mut self) -> crossterm::Result<()> {
        self.item_list_model.filter_items();
        self.item_list.render(self.item_list_model)?;

        loop {
            if let Event::Key(key_event) = read().unwrap() {
                match key_event.code {
                    KeyCode::Enter => { return self.execute_selection(); }
                    KeyCode::Esc => { return self.item_list.remove(self.item_list_model); }
                    KeyCode::Down => { self.on_selection_change(1)? }
                    KeyCode::Up => { self.on_selection_change(-1)? }
                    KeyCode::Backspace => { self.on_backspace()? }
                    KeyCode::Char(ch) => { self.on_character_entered(ch)? }
                    _ => {}
                }
            }
        }
    }

    fn on_selection_change(&mut self, direction: i16) -> crossterm::Result<()> {
        if self.item_list_model.change_selection(direction) {
            return self.item_list.refresh(self.item_list_model);
        }
        Ok(())
    }

    fn on_backspace(&mut self)  -> crossterm::Result<()> {
        if self.item_list_model.pop_search_term() {
            return self.item_list.refresh(self.item_list_model);
        }
        Ok(())
    }

    fn on_character_entered(&mut self, ch: char) -> crossterm::Result<()> {
        self.item_list_model.add_to_search_term(ch);
        self.item_list.refresh(self.item_list_model)

    }

    fn execute_selection(&mut self) -> crossterm::Result<()> {
        self.item_list.remove(self.item_list_model)?;
        let command = self.item_list_model.get_selected_item().to_string();
        self.print_command_info(&command);
        disable_raw_mode()?;
        execute_command(&command);
        self.item_list.reset_cursor_column()?;
        Ok(())
    }

    fn print_command_info(&self, command: &str) {
        println!("Executing {}\r", &command.green());
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

