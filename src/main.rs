use console::{Key, Term};
use crossterm_cursor::{TerminalCursor, cursor};

fn main() -> std::io::Result<()> {
    let term = Term::stdout();
    let mut search_term = String::from("");

    let items = vec!("This is item 1", "This is another item", "And another");
    let mut renderer = ItemListRenderer::new(&term, &items);
    renderer.render_items(&search_term)?;

    let mut cursor = cursor();
    cursor.move_up(items.len() as u16 + 1);
    cursor.move_right(2);

    let delete = char::from(127);

    loop {
        let (cursor_x, _) = cursor.pos();
        let key = term.read_key().unwrap();

        match key {
            Key::Escape => { return Ok(()); }

            Key::Char(ch) => {
                if ch == delete && cursor_x > 2 {
                    search_term.pop();
                    renderer.refresh_items(&search_term)?;
                    cursor.move_left(1);
                }
                else if ch != delete {
                    search_term.push(ch);
                    renderer.refresh_items(&search_term)?;
                    cursor.move_right(1);
                }
            }

            _ => {}
        }
    }
}

struct ItemListRenderer<'a> {
    term: &'a Term,
    items: &'a Vec<&'a str>,
    cursor: TerminalCursor
}

impl<'a> ItemListRenderer<'a> {
    pub fn new(term: &'a Term, items: &'a Vec<&str>) -> ItemListRenderer<'a> {
        ItemListRenderer {term: term, items: items, cursor: cursor()}
    }

    pub fn refresh_items(&mut self, search_term: &str) -> std::io::Result<()> {
        self.cursor.save_position()?;
        self.cursor.move_down(self.items.len() as u16 + 1);

        self.term.clear_last_lines(self.items.len() + 1)?;
        self.render_items(&search_term)?;
        self.cursor.reset_position()?;

        Ok(())
    }

    pub fn render_items(&self, search_term: &str) -> std::io::Result<()> {
        self.term.write_line(&format!("> {}", search_term))?;
        for item in self.items.iter() {
            if item.find(search_term) != None {
                self.term.write_line(&format!("{}", item))?;
            }

        }
        Ok(())
    }

}
