use console::{Key, Term};
use crossterm_cursor::{TerminalCursor, cursor};

fn main() -> std::io::Result<()> {
    let term = Term::stdout();
    let mut search_term = String::from("");

    let items = vec!("This is item 1", "This is another item", "And another");
    let mut renderer = ItemList::new(&term, &items);
    renderer.render(&search_term)?;
    renderer.init_cursor()?;

    let delete = char::from(127);

    loop {
        let key = term.read_key().unwrap();

        match key {
            Key::Escape => { return Ok(()); }

            Key::Char(ch) => {
                if ch == delete && search_term.len() > 0 {
                    search_term.pop();
                    renderer.refresh(&search_term)?;
                }
                else if ch != delete {
                    search_term.push(ch);
                    renderer.refresh(&search_term)?;
                }
            }

            _ => {}
        }
    }
}

struct ItemList<'a> {
    term: &'a Term,
    items: &'a Vec<&'a str>,
    cursor: TerminalCursor
}

impl<'a> ItemList<'a> {
    pub fn new(term: &'a Term, items: &'a Vec<&str>) -> ItemList<'a> {
        ItemList {term: term, items: items, cursor: cursor()}
    }

    pub fn refresh(&mut self, search_term: &str) -> std::io::Result<()> {
        self.cursor.save_position()?;
        self.cursor.move_down(self.height());

        self.term.clear_last_lines(self.height() as usize)?;
        self.render(&search_term)?;
        self.cursor.reset_position()?;
        let (_, y) = self.cursor.pos();
        self.cursor.goto(search_term.len() as u16 + 2, y)?;

        Ok(())
    }

    pub fn render(&self, search_term: &str) -> std::io::Result<()> {
        self.term.write_line(&format!("> {}", search_term))?;
        for item in self.items.iter() {
            if item.find(search_term) != None {
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

    fn height(&self) -> u16 {
        self.items.len() as u16 + 1
    }

}
