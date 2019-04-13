
use std::fs::File;
use std::io::{BufRead, BufReader};
use console::{Key, Term, style};
use crossterm_cursor::{TerminalCursor, cursor};

fn main() -> std::io::Result<()> {
    let items = read_items("Cargo.toml");
    let term = Term::stdout();
    let mut search_term = String::from("");

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

fn read_items(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    let mut items = Vec::new();

    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        items.push(line);
    }

    items
}

struct ItemList<'a> {
    term: &'a Term,
    items: &'a Vec<String>,
    cursor: TerminalCursor
}

impl<'a> ItemList<'a> {
    pub fn new(term: &'a Term, items: &'a Vec<String>) -> ItemList<'a> {
        ItemList {
            term: term, 
            items: items, 
            cursor: cursor()}
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
        for (index, item) in self.items.iter()
                                .filter(|it| it.find(search_term) != None )
                                .enumerate() {
            if index == 1 {
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

    fn height(&self) -> u16 {
        self.items.len() as u16 + 1
    }

}
