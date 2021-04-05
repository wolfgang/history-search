use std::io::Write;

use crossterm::{
    cursor::{MoveToColumn, MoveUp, RestorePosition, SavePosition},
    execute,
};
use crossterm::style::{StyledContent, Styler};
use crossterm::terminal::size;

use crate::item_list_model::ItemListModel;

pub struct ItemListView<'a, T> where T: Write {
    stdout: &'a mut T,
    current_height: u16,
}

impl<'a, T> ItemListView<'a, T> where T: Write {
    pub fn new(stdout: &'a mut T) -> Self {
        Self { stdout, current_height: 0 }
    }
    pub fn remove(&mut self) -> crossterm::Result<()> {
        self.clear()?;
        self.reset_cursor_column()
    }

    pub fn reset_cursor_column(&mut self) -> crossterm::Result<()> {
        execute!(self.stdout, MoveToColumn(0))
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
        execute!(self.stdout, MoveToColumn(0))?;
        let (cols, _) = size()?;
        let blank_line = " ".repeat(cols as usize - 1);
        for _ in 0..self.current_height {
            println!("{}\r", blank_line);
        }
        let rows = self.current_height;
        execute!(self.stdout, MoveUp(rows))
    }


    fn render(&mut self, model: &ItemListModel) -> crossterm::Result<()> {
        execute!(self.stdout, MoveToColumn(0), SavePosition)?;
        println!("> {}\r", model.get_search_term());
        for (item, is_selected) in model.filtered_items_iter() {
            println!("{}\r", Self::printable_item(item, is_selected));
        }
        execute!(self.stdout,RestorePosition,MoveToColumn(model.get_search_term().len() as u16 + 3))
    }

    fn printable_item(item: &String, is_selected: bool) -> StyledContent<String> {
        if is_selected { item.clone().reverse() } else { item.clone().reset() }
    }
}
