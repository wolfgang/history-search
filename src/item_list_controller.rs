use std::env;
use std::io::Write;
use std::process::Command;

use crossterm::event::{Event, KeyCode, read};
use crossterm::style::Colorize;
use crossterm::terminal::disable_raw_mode;

use crate::item_list_model::ItemListModel;
use crate::item_list_view::ItemListView;

pub struct ItemListController<'a, T> where T: Write {
    item_list: &'a mut ItemListView<'a, T>,
    item_list_model: &'a mut ItemListModel<'a>,
}

impl<'a, T> ItemListController<'a, T> where T: Write {
    pub fn new(item_list: &'a mut ItemListView<'a, T>, item_list_model: &'a mut ItemListModel<'a>) -> ItemListController<'a, T> {
        ItemListController { item_list, item_list_model }
    }

    pub fn run(&mut self) -> crossterm::Result<()> {
        self.refresh_item_list()?;

        loop {
            if let Event::Key(key_event) = read().unwrap() {
                match key_event.code {
                    KeyCode::Enter => { return self.on_enter(); }
                    KeyCode::Esc => { return self.on_cancel(); }
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
            return self.refresh_item_list();
        }
        Ok(())
    }

    fn on_backspace(&mut self) -> crossterm::Result<()> {
        if self.item_list_model.pop_search_term() {
            return self.refresh_item_list();
        }
        Ok(())
    }

    fn on_character_entered(&mut self, ch: char) -> crossterm::Result<()> {
        self.item_list_model.add_to_search_term(ch);
        self.refresh_item_list()
    }

    fn on_cancel(&mut self) -> crossterm::Result<()> {
        self.item_list.remove()
    }

    fn on_enter(&mut self) -> crossterm::Result<()> {
        self.item_list.remove()?;
        let command = self.item_list_model.get_selected_item().to_string();
        self.print_command_info(&command);
        disable_raw_mode()?;
        execute_command(&command);
        self.item_list.reset_cursor_column()
    }

    fn refresh_item_list(&mut self) -> crossterm::Result<()> {
        let count = self.item_list.get_renderable_items_count(self.item_list_model);
        self.item_list_model.set_selection_window_height(count);
        self.item_list.refresh(self.item_list_model)
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


// fn execute_command(command: &str) {
//     let shell = env::var_os("SHELL").unwrap();
//     let get_output = Command::new(shell)
//         .arg("-c")
//         .arg(&format!("echo {}", &command))
//         .output()
//         .expect("Failed to execute");
//
//     let command2 = from_utf8(&get_output.stdout).unwrap();
//
//     let parts: Vec<&str> = command2.trim().split_ascii_whitespace().collect();
//     let cmd = &parts[0];
//     Command::new(&cmd)
//         .args(&parts[1..])
//         .status()
//         .expect(&format!("Failed to execute: {}", command));
// }

