use std::cmp::{max, min};
use std::io::stdout;

use crossterm::{
    cursor::{MoveRight, MoveToColumn, MoveUp, RestorePosition, SavePosition},
    execute
};
use crossterm::style::Styler;
use crossterm::terminal::size;

pub struct ItemList<'a> {
    items: &'a Vec<String>,
    search_term: String,
    selection: i16,
    selection_window_start: i16,
    selection_window_y: i16,
    filtered_items: Vec<&'a String>,
}

impl<'a> ItemList<'a> {
    pub fn new(items: &'a Vec<String>) -> ItemList<'a> {
        ItemList {
            items,
            search_term: String::new(),
            filtered_items: Vec::with_capacity(10),
            selection: 0,
            selection_window_start: 0,
            selection_window_y: 0,
        }
    }

    pub fn init(&mut self) -> crossterm::Result<()> {
        self.filter_items();
        self.render()?;
        self.init_cursor()
    }

    pub fn remove(&mut self) -> crossterm::Result<()> {
        self.clear()?;
        self.reset_cursor()
    }

    pub fn reset_cursor(&mut self) -> crossterm::Result<()> {
        execute!(stdout(), MoveToColumn(0))
    }

    pub fn on_backspace(&mut self) -> crossterm::Result<()> {
        if !self.search_term.is_empty() {
            self.search_term.pop();
            return self.process_input();
        }
        Ok(())
    }

    pub fn on_character_entered(&mut self, ch: char) -> crossterm::Result<()> {
        self.search_term.push(ch);
        self.process_input()
    }

    fn process_input(&mut self) -> crossterm::Result<()> {
        self.clear()?;
        self.filter_items();
        self.selection = 0;
        self.selection_window_start = 0;
        self.selection_window_y = 0;
        self.refresh()?;
        Ok(())
    }

    pub fn change_selection(&mut self, direction: i16) -> crossterm::Result<()> {
        let num_items = self.filtered_items.len() as i16;
        let prev_selection = self.selection;
        self.selection = max(0, min(num_items - 1, self.selection + direction));
        if prev_selection != self.selection {
            self.selection_window_y = self.selection_window_y + direction;
            if self.selection_window_y < 0 {
                self.selection_window_y = 0;
                self.selection_window_start = max(0, self.selection_window_start + direction);
            }

            if self.selection_window_y > 9 {
                self.selection_window_y = 9;
                self.selection_window_start = min(self.selection_window_start + direction, num_items - 10);
            }

            self.refresh()?;
        }

        Ok(())
    }

    pub fn refresh(&mut self) -> crossterm::Result<()> {
        execute!(stdout(), MoveToColumn(0), SavePosition)?;
        self.clear()?;
        self.render()?;
        execute!(stdout(), RestorePosition, MoveRight(self.search_term.len() as u16 + 2))?;
        Ok(())
    }

    pub fn clear(&mut self) -> crossterm::Result<()> {
        execute!(stdout(),SavePosition)?;
        let (cols, _) = size().unwrap();

        let blank_line = " ".repeat((cols - 10) as usize);
        for _ in 0..self.height() {
            println!("{}\r", blank_line);
        }
        execute!(stdout(),RestorePosition)?;

        Ok(())
    }

    pub fn render(&self) -> crossterm::Result<()> {
        println!("> {}\r", self.search_term);
        for index in self.selection_window_start..self.get_selection_window_end() {
            let item = self.filtered_items[index as usize];
            if index == self.selection as i16 {
                println!("{}\r", item.clone().reverse());
            } else {
                println!("{}\r", item);
            }
        }
        Ok(())
    }

    pub fn init_cursor(&mut self) -> crossterm::Result<()> {
        execute!(stdout(), MoveUp(self.height()), MoveRight(2))?;
        Ok(())
    }

    pub fn selected_item(&self) -> &String {
        match self.filtered_items.get(self.selection as usize) {
            Some(item) => { item }
            None => { &self.search_term }
        }
    }

    fn filter_items(&mut self) {
        let search_term_upper = self.search_term.to_ascii_uppercase();
        self.filtered_items = self.items.iter()
            .filter(|it| it.to_ascii_uppercase().find(&search_term_upper) != None)
            .collect()
    }

    fn height(&self) -> u16 {
        let (cols, _) = size().unwrap();
        let mut result = 0;

        for index in self.selection_window_start..self.get_selection_window_end() {
            let item = self.filtered_items[index as usize];
            let l = item.len();
            result = result + (l as f64 / cols as f64).ceil() as u16;
        }

        result + 1
    }

    fn get_selection_window_end(&self) -> i16 {
        min(self.filtered_items.len() as i16, self.selection_window_start + 10)
    }
}
