use std::io::stdout;

use crossterm::{
    cursor::{MoveToColumn, MoveUp, RestorePosition, SavePosition},
    execute,
};
use crossterm::style::Styler;
use crossterm::terminal::size;

use crate::item_list_model::ItemListModel;

pub struct ItemList {
}

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

    fn clear(&mut self, model: &ItemListModel) -> crossterm::Result<()> {
        execute!(stdout(),SavePosition, MoveToColumn(0))?;
        let (cols, _) = size().unwrap();
        let blank_line = " ".repeat(cols as usize - 1);
        for _ in 0..model.get_max_height() {
            println!("{}\r", blank_line);
        }
        execute!(stdout(),RestorePosition)?;

        Ok(())
    }

    pub fn render(&self, model: &ItemListModel) -> crossterm::Result<()> {
        execute!(stdout(), MoveToColumn(0))?;
        println!("> {}\r", model.get_search_term());

        for (item, is_selected) in model.filtered_items_iter() {
            if is_selected {
                println!("{}\r", item.clone().reverse());
            } else {
                println!("{}\r", item);
            }
        }

        execute!(
            stdout(),
            MoveUp(model.get_filtered_height()),
            MoveToColumn(model.get_search_term().len() as u16 + 3))
    }
}
