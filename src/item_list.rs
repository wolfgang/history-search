use std::io::stdout;

use crossterm::{
    cursor::{MoveToColumn, MoveUp, RestorePosition, SavePosition},
    execute,
};
use crossterm::style::{Styler, StyledContent};
use crossterm::terminal::size;

use crate::item_list_model::ItemListModel;

pub struct ItemList {}

impl ItemList {
    pub fn remove(&mut self, model: &ItemListModel) -> crossterm::Result<()> {
        self.clear(model)?;
        self.reset_cursor_column()
    }

    pub fn reset_cursor_column(&mut self) -> crossterm::Result<()> {
        execute!(stdout(), MoveToColumn(0))
    }

    pub fn refresh(&mut self, model: &ItemListModel) -> crossterm::Result<()> {
        self.clear(model)?;
        self.render(model)
    }

    pub fn render(&self, model: &ItemListModel) -> crossterm::Result<()> {
        execute!(stdout(), MoveToColumn(0), SavePosition)?;
        println!("> {}\r", model.get_search_term());

        for (item, is_selected) in model.filtered_items_iter() {
            println!("{}\r", Self::printable_item(item, is_selected));
        }

        execute!(
            stdout(),
            RestorePosition,
            MoveToColumn(model.get_search_term().len() as u16 + 3))
    }

    fn clear(&mut self, model: &ItemListModel) -> crossterm::Result<()> {
        execute!(stdout(), MoveToColumn(0))?;
        let (cols, _) = size()?;
        let blank_line = " ".repeat(cols as usize - 1);
        for _ in 0..model.get_max_height() {
            println!("{}\r", blank_line);
        }
        execute!(stdout(), MoveUp(model.get_max_height()))
    }

    fn printable_item(item: &String, is_selected: bool) -> StyledContent<String> {
        if is_selected { item.clone().reverse() } else { item.clone().reset() }
    }
}
