use std::io::Write;

use crossterm::{
    cursor::{MoveToColumn, MoveUp, RestorePosition, SavePosition},
    execute,
};
use crossterm::style::{StyledContent, Styler};

use crate::item_list_model::ItemListModel;

pub struct ItemListView<'a, T> where T: Write {
    stdout: &'a mut T,
}

impl<'a, T> ItemListView<'a, T> where T: Write {
    pub fn new(stdout: &'a mut T) -> Self {
        Self { stdout }
    }
    pub fn remove(&mut self, display_width: u16, model: &ItemListModel) -> crossterm::Result<()> {
        self.clear(display_width, model)?;
        self.reset_cursor_column()
    }

    pub fn reset_cursor_column(&mut self) -> crossterm::Result<()> {
        execute!(self.stdout, MoveToColumn(0))
    }

    pub fn refresh(&mut self, display_width: u16, model: &ItemListModel) -> crossterm::Result<()> {
        self.clear(display_width, model)?;
        self.render(model)
    }

    fn clear(&mut self, display_width: u16, model: &ItemListModel) -> crossterm::Result<()> {
        execute!(self.stdout, MoveToColumn(0))?;
        let blank_line = " ".repeat(display_width as usize);
        let rows = (model.get_selection_window_height() + 1) as u16;
        for _ in 0..rows {
            self.stdout.write_fmt(format_args!("{}\n\r", blank_line))?;
        }
        execute!(self.stdout, MoveUp(rows))
    }


    pub fn render(&mut self, model: &ItemListModel) -> crossterm::Result<()> {
        execute!(self.stdout, MoveToColumn(0), SavePosition)?;
        self.stdout.write_fmt(format_args!("> {}\n\r", model.get_search_term()))?;
        for (item, is_selected) in model.filtered_items_iter() {
            self.stdout.write_fmt(format_args!("{}\n\r", Self::printable_item(item, is_selected)))?;
        }
        execute!(self.stdout,RestorePosition,MoveToColumn(model.get_search_term().len() as u16 + 3))
    }

    fn printable_item(item: &String, is_selected: bool) -> StyledContent<String> {
        if is_selected { item.clone().reverse() } else { item.clone().reset() }
    }
}
