
use std::cmp::{min, max};
use console::{Term, style};
use crossterm_cursor::{TerminalCursor, cursor};

pub struct ItemList<'a> {
    term: &'a Term,
    items: &'a Vec<String>,
    search_term: String,
    selection: i16,
    cursor: TerminalCursor,
}

impl<'a> ItemList<'a> {
    pub fn new(term: &'a Term, items: &'a Vec<String>) -> ItemList<'a> {
        ItemList {
            term: term, 
            items: items, 
            search_term: String::new(),
            selection: 0,
            cursor: cursor()}
    }

    pub fn on_character_entered(&mut self, ch: char) -> std::io::Result<()> {
        let delete = char::from(127);

        if ch == delete && !self.search_term.is_empty() {
            self.search_term.pop();
        }
        else if ch != delete {
            self.search_term.push(ch);
        }
        self.selection = 0;
        self.refresh()?;
        Ok(())

    }

    pub fn change_selection(&mut self, direction: i16) -> std::io::Result<()> {
        let max_selection = self.filtered_items().len() as i16 -1;
        self.selection = max(0, min(max_selection, self.selection + direction));
        self.refresh()?;
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
        for (index, item) in self.filtered_items().iter().enumerate() {
            if index == self.selection as usize {
                self.term.write_line(&format!("{}", style(item).reverse()))?;
            }
            else {
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
        match self.filtered_items().get(self.selection as usize) {
            Some(item) => { item }
            None => { &self.search_term }
        }
    }

    fn filtered_items(&self) -> Vec<&String> {
        self.items.iter().filter(|it| it.find(&self.search_term) != None ).collect()
    }

    fn height(&self) -> u16 {
        let (width, _) = self.term.size();
        let mut result = 0;
        for item in self.items.iter() {
            let l = item.len();
            result = result + (l as f64/width as f64).ceil() as u16;
        }

        result + 1
    }

}
