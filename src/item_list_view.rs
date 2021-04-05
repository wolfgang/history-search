use std::io::stdout;

use crossterm::{
    cursor::{MoveToColumn, MoveUp, RestorePosition, SavePosition},
    execute,
};
use crossterm::style::{StyledContent, Styler};
use crossterm::terminal::size;

use crate::item_list_model::ItemListModel;

pub struct ItemListView {
    current_height: u16
}

impl ItemListView {
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
        self.prepare_next_frame(model)?;
        self.clear()?;
        self.render(model)
    }

    fn prepare_next_frame(&mut self, model: &ItemListModel) -> crossterm::Result<()> {
        // Count input line
        let mut current_height = 1;
        let mut count = 0;
        let (cols, _) = size()?;
        for (item, _) in model.filtered_items_iter() {
            current_height = current_height + (item.len() as f64 / cols as f64).ceil() as u16;
            count += 1;
        }

        current_height += (model.get_selection_window_height() - count) as u16;

        self.current_height = current_height;
        Ok(())
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


    fn render(&mut self, model: &ItemListModel) -> crossterm::Result<()> {
        execute!(stdout(), MoveToColumn(0), SavePosition)?;
        println!("> {}\r", model.get_search_term());
        for (item, is_selected) in model.filtered_items_iter() {
            println!("{}\r", Self::printable_item(item, is_selected));
        }
        execute!(stdout(),RestorePosition,MoveToColumn(model.get_search_term().len() as u16 + 3))
    }

    fn printable_item(item: &String, is_selected: bool) -> StyledContent<String> {
        if is_selected { item.clone().reverse() } else { item.clone().reset() }
    }
}
