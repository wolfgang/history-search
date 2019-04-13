
use std::fs::File;
use std::io::{BufRead, BufReader};
use console::{Key, Term, style};
use crossterm_cursor::{TerminalCursor, cursor};

fn main() -> std::io::Result<()> {
    let items = read_items("Cargo.toml");
    let term = Term::stdout();

    let mut item_list = ItemList::new(&term, &items);
    item_list.render()?;
    item_list.init_cursor()?;

    loop {
        let key = term.read_key().unwrap();

        match key {
            Key::Enter => {
                item_list.clear()?;
                println!("Selected: {:}", item_list.selected_item());
                return Ok(());
            }
            Key::Escape => { return Ok(()); }
            Key::ArrowUp => { item_list.change_selection(-1)?; }
            Key::ArrowDown => { item_list.change_selection(1)?; }
            Key::Char(ch) => { item_list.on_character_entered(ch)?; }
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

        if ch == delete && self.search_term.len() > 0 {
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
        self.selection = self.selection + direction;
        if self.selection < 0 { self.selection = 0; }
        let max_selection = self.items.len() as i16 -1;
        if self.selection > max_selection { self.selection = max_selection; }
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
        self.filtered_items()[self.selection as usize]
    }

    fn filtered_items(&self) -> Vec<&String> {
        self.items.iter().filter(|it| it.find(&self.search_term) != None ).collect()
    }

    fn height(&self) -> u16 {
        self.items.len() as u16 + 1
    }

}
