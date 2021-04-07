use std::env;
use std::io::Write;
use std::process::Command;

use crossterm::event::{KeyCode, KeyEvent};
use crossterm::style::Colorize;
use crossterm::terminal::disable_raw_mode;

use crate::item_list_model::ItemListModel;
use crate::item_list_view::ItemListView;

pub struct ItemListController<'a, T> where T: Write {
    item_list: &'a mut ItemListView<T>,
    item_list_model: &'a mut ItemListModel,
}

impl<'a, T> ItemListController<'a, T> where T: Write {
    pub fn new(item_list: &'a mut ItemListView<T>, item_list_model: &'a mut ItemListModel) -> ItemListController<'a, T> {
        ItemListController { item_list, item_list_model }
    }

    pub fn init(&mut self) -> crossterm::Result<()> {
        self.refresh_item_list()
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> crossterm::Result<bool> {
        match key_event.code {
            KeyCode::Enter => { return self.on_enter(); }
            KeyCode::Esc => { return self.on_cancel(); }
            KeyCode::Down => { return self.on_selection_change(1); }
            KeyCode::Up => { return self.on_selection_change(-1); }
            KeyCode::Backspace => { return self.on_backspace(); }
            KeyCode::Char(ch) => { return self.on_character_entered(ch); }
            _ => {}
        }
        Ok(true)
    }


    fn on_selection_change(&mut self, direction: i16) -> crossterm::Result<bool> {
        if self.item_list_model.change_selection(direction) {
            self.refresh_item_list()?;
        }
        Ok(true)
    }

    fn on_backspace(&mut self) -> crossterm::Result<bool> {
        if self.item_list_model.pop_search_term() {
            self.refresh_item_list()?;
        }
        Ok(true)
    }

    fn on_character_entered(&mut self, ch: char) -> crossterm::Result<bool> {
        self.item_list_model.add_to_search_term(ch);
        self.refresh_item_list()?;
        Ok(true)
    }

    fn on_cancel(&mut self) -> crossterm::Result<bool> {
        self.item_list.remove()?;
        Ok(false)
    }

    fn on_enter(&mut self) -> crossterm::Result<bool> {
        self.item_list.remove()?;
        let command = self.item_list_model.get_selected_item().to_string();
        self.print_command_info(&command);
        disable_raw_mode()?;
        execute_command(&command);
        self.item_list.reset_cursor_column()?;
        Ok(false)
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

