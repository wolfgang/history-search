use std::io::Write;

use crossterm::{
    cursor::{MoveToColumn, MoveUp, RestorePosition, SavePosition},
    execute,
};
use crossterm::style::{StyledContent, Styler};

use crate::item_list_model::ItemListModel;

pub struct ItemListView<'a, T> where T: Write {
    max_renderable_items: u16,
    stdout: &'a mut T,
}

impl<'a, T> ItemListView<'a, T> where T: Write {
    pub fn new(max_renderable_items: u16, stdout: &'a mut T) -> Self {
        Self { max_renderable_items, stdout }
    }

    pub fn get_renderable_items_count(&self, display_width: u16, model: &ItemListModel) -> u16 {
        let max_height = self.max_renderable_items;
        let mut current_height = 0;
        let mut count: u16 = 0;
        for (item, _) in model.filtered_items_iter() {
            let line_height = (item.len() as f64 / display_width as f64).ceil() as u16;
            if current_height + line_height > max_height { break; }
            current_height += line_height;
            count += 1;
        }

        count
    }

    pub fn remove(&mut self, display_width: u16) -> crossterm::Result<()> {
        self.clear(display_width)?;
        self.reset_cursor_column()
    }

    pub fn reset_cursor_column(&mut self) -> crossterm::Result<()> {
        execute!(self.stdout, MoveToColumn(0))
    }

    pub fn refresh(&mut self, display_width: u16, model: &ItemListModel) -> crossterm::Result<()> {
        self.clear(display_width)?;
        self.render(model)
    }

    fn clear(&mut self, display_width: u16) -> crossterm::Result<()> {
        execute!(self.stdout, MoveToColumn(0))?;
        let blank_line = " ".repeat(display_width as usize);
        let rows = self.max_renderable_items + 1;
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

    pub fn get_max_lines(&self) -> u16 {
        self.max_renderable_items
    }

    fn printable_item(item: &String, is_selected: bool) -> StyledContent<String> {
        if is_selected { item.clone().reverse() } else { item.clone().reset() }
    }
}
