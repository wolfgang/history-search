use crate::item_list::ItemList;

use std::process::Command;
use std::env;

use console::{Key, Term, Style};

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
        let (working_dir, command) = self.parse_selection();

        self.print_command_info(&working_dir, &command)?;
        execute_command(&working_dir, &command);
        Ok(())
    }

    fn parse_selection(&self) -> (String, String) {
        let item = self.item_list.selected_item();
        let parts : Vec<&str> = item.split("]").collect();
        if parts.len() == 2 {
            return (parts[0][1..].to_string(), parts[1].to_string());
        }
        (String::from("."), parts[0].to_string())
    }

    fn print_command_info(&self, working_dir: &str, command: &str) -> std::io::Result<()> {
        let green = Style::new().green(); 
        let blue = Style::new().blue(); 

        let prefix = if working_dir != "." { 
            String::from(format!(" [{}] ", &working_dir)) 
        } else { 
            String::from(" ") 
        };

        let info = format!("->{}{}", blue.apply_to(prefix), green.apply_to(&command));
        self.term.write_line(&info)
    }

}

fn execute_command(working_dir: &str, command: &str) {
    let shell = env::var_os("SHELL").unwrap();
    Command::new(shell)
            .arg("-c")
            .arg(&command)
            .current_dir(&working_dir)
            .status()
            .expect(&format!("Failed to execute: {}", command ));
}

