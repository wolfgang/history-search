use std::cmp::{max, min};

use console::{style, Term};
use crossterm_cursor::{cursor, TerminalCursor};

pub struct ItemList<'a> {
    term: &'a Term,
    items: &'a Vec<String>,
    search_term: String,
    selection: i16,
    selection_window_start: i16,
    selection_window_y: i16,
    cursor: TerminalCursor,
    filtered_items: Vec<&'a String>,
}

impl<'a> ItemList<'a> {
    pub fn new(term: &'a Term, items: &'a Vec<String>) -> ItemList<'a> {
        ItemList {
            term: term,
            items: items,
            search_term: String::new(),
            filtered_items: Vec::with_capacity(10),
            selection: 0,
            selection_window_start: 0,
            selection_window_y: 0,
            cursor: cursor(),
        }
    }

    pub fn init(&mut self) {
        self.filter_items();
    }

    pub fn on_character_entered(&mut self, ch: char) -> std::io::Result<()> {
        let delete = char::from(127);
        if ch == delete && !self.search_term.is_empty() {
            self.search_term.pop();
        } else if ch != delete {
            self.search_term.push(ch);
        }

        self.clear()?;
        self.filter_items();
        self.selection = 0;
        self.selection_window_start = 0;
        self.selection_window_y = 0;
        self.refresh()?;
        Ok(())
    }

    pub fn change_selection(&mut self, direction: i16) -> std::io::Result<()> {
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

    pub fn refresh(&mut self) -> std::io::Result<()> {
        self.cursor.save_position()?;
        self.clear()?;
        self.render()?;
        self.cursor.reset_position()?;
        let (_, y) = self.cursor.pos();
        self.cursor.goto(self.search_term.len() as u16 + 2, y)?;

        Ok(())
    }

    pub fn clear(&mut self) -> std::io::Result<()> {
        self.cursor.move_down(self.height());
        self.term.clear_last_lines(self.height() as usize)?;
        Ok(())
    }

    pub fn render(&self) -> std::io::Result<()> {
        self.term.write_line(&format!("> {}", self.search_term))?;
        for index in self.selection_window_start..self.get_selection_window_end() {
            let item = self.filtered_items[index as usize];
            if index == self.selection as i16 {
                self.term.write_line(&format!("{}", style(item).reverse()))?;
            } else {
                self.term.write_line(&format!("{}", item))?;
            }
        }
        Ok(())
    }

    pub fn init_cursor(&mut self) -> std::io::Result<()> {
        self.cursor.move_up(self.height());
        self.cursor.move_right(2);
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
        let (_, width) = self.term.size();
        let mut result = 0;

        for index in self.selection_window_start..self.get_selection_window_end() {
            let item = self.filtered_items[index as usize];
            let l = item.len();
            result = result + (l as f64 / width as f64).ceil() as u16;
        }

        result + 1
    }

    fn get_selection_window_end(&self) -> i16 {
        min(self.filtered_items.len() as i16, self.selection_window_start + 10)
    }
}
