use std::cell::RefCell;
use std::cmp::min;
use std::io::Write;
use std::rc::Rc;

use crossterm::{
    cursor::{MoveToColumn, MoveUp, RestorePosition, SavePosition},
    execute,
};
use crossterm::style::{StyledContent, Styler};

use crate::item_list_model::ItemListModel;

pub struct ItemListView<T> where T: Write {
    display_width: u16,
    display_height: u16,
    stdout_ref: Rc<RefCell<T>>,
}

impl<T> ItemListView<T> where T: Write {
    pub fn new(
        display_width: u16,
        display_height: u16,
        stdout_ref: Rc<RefCell<T>>,
    ) -> Self
    {
        Self {
            display_height,
            display_width,
            stdout_ref,
        }
    }

    pub fn get_renderable_items_count(&self) -> u16 {
        self.display_height - 1
    }

    pub fn remove(&mut self) -> crossterm::Result<()> {
        self.clear()?;
        self.reset_cursor_column()
    }

    pub fn reset_cursor_column(&mut self) -> crossterm::Result<()> {
        let mut stdout = self.stdout_ref.borrow_mut();
        execute!(stdout, MoveToColumn(0))
    }

    pub fn refresh(&mut self, model: &ItemListModel) -> crossterm::Result<()> {
        self.clear()?;
        self.render(model)
    }

    fn clear(&mut self) -> crossterm::Result<()> {
        let mut stdout = self.stdout_ref.borrow_mut();
        execute!(stdout, MoveToColumn(0))?;
        let blank_line = " ".repeat(self.display_width as usize);
        let rows = self.display_height;
        for _ in 0..rows {
            stdout.write_fmt(format_args!("{}\n\r", blank_line))?;
        }
        execute!(stdout, MoveUp(rows))
    }


    fn render(&mut self, model: &ItemListModel) -> crossterm::Result<()> {
        let mut stdout = self.stdout_ref.borrow_mut();
        execute!(stdout, MoveToColumn(0), SavePosition)?;
        stdout.write_fmt(format_args!("> {}\n\r", model.get_search_term()))?;
        for (item, is_selected) in model.selectable_items_iter() {
            stdout.write_fmt(format_args!("{}\n\r", self.printable_item(&item, is_selected)))?;
        }
        execute!(stdout,RestorePosition,MoveToColumn(model.get_search_term().len() as u16 + 3))
    }

    pub fn get_max_lines(&self) -> u16 {
        self.display_height
    }

    fn printable_item(&self, item: &String, is_selected: bool) -> StyledContent<String> {
        let len = item.len();
        let str = item.clone().as_str()[..min(self.display_width, len as u16) as usize].to_string();
        if is_selected { str.reverse() } else { str.reset() }
    }
}
