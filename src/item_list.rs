use std::io::stdout;

use crossterm::{
    cursor::{MoveToColumn, MoveUp, RestorePosition, SavePosition},
    execute,
};
use crossterm::style::{StyledContent, Styler};
use crossterm::terminal::size;

use crate::item_list_model::ItemListModel;

pub struct ItemList {
    current_height: u16
}

impl ItemList {
    pub fn new() -> Self {
        Self { current_height: 0 }
    }
    pub fn remove(&mut self) -> crossterm::Result<()> {
        self.clear()?;
        self.reset_cursor_column()
    }

    pub fn reset_cursor_column(&mut self) -> crossterm::Result<()> {
        execute!(stdout(), MoveToColumn(0))
    }

    pub fn refresh(&mut self, model: &ItemListModel) -> crossterm::Result<()> {
        self.clear()?;
        self.render(model, false)
    }

    pub fn render(&mut self, model: &ItemListModel, dry_run: bool) -> crossterm::Result<()> {
        let (cols, _) = size()?;
        execute!(stdout(), MoveToColumn(0), SavePosition)?;
        if !dry_run {
            println!("> {}\r", model.get_search_term());
        }

        let mut result = 1;

        for (item, is_selected) in model.filtered_items_iter() {
            if !dry_run { println!("{}\r", Self::printable_item(item, is_selected)) };
            result = result + (item.len() as f64 / cols as f64).ceil() as u16;
        }

        self.current_height = result;
        execute!(stdout(),RestorePosition,MoveToColumn(model.get_search_term().len() as u16 + 3))
    }

    fn clear(&mut self) -> crossterm::Result<()> {
        execute!(stdout(), MoveToColumn(0))?;
        let (cols, _) = size()?;
        let blank_line = " ".repeat(cols as usize - 1);
        for _ in 0..self.current_height {
            println!("{}\r", blank_line);
        }
        execute!(stdout(), MoveUp(self.current_height))
    }

    fn printable_item(item: &String, is_selected: bool) -> StyledContent<String> {
        if is_selected { item.clone().reverse() } else { item.clone().reset() }
    }
}
