use console::{Key, Term};
use crossterm_cursor::{TerminalCursor, cursor};

fn main() -> std::io::Result<()> {
    let term = Term::stdout();
    let mut search_term = String::from("");

    let items = vec!("This is item 1", "This is another item", "And another");

    render_items(&term, &items, &search_term)?;
    let mut cursor = cursor();
    cursor.move_up(items.len() as u16 + 1);
    cursor.move_right(2);


    loop {
        let key = term.read_key().unwrap();

        match key {
            Key::Escape => { return Ok(()); }

            Key::Char(ch) => {
                if ch == char::from(127) {
                    search_term.pop();
                    refresh_items(&term, &mut cursor, &items, &search_term)?;
                    cursor.move_left(1);
                }
                else {
                    search_term.push(ch);
                    refresh_items(&term, &mut cursor, &items, &search_term)?;
                    cursor.move_right(1);
                }
            }

            _ => {}
        }
    }
}

fn refresh_items(
    term: &Term,
    cursor: &mut TerminalCursor,
    items: &Vec<&str>, 
    search_term: &str) -> std::io::Result<()> {

    cursor.save_position()?;
    cursor.move_down(items.len() as u16 + 1);

    term.clear_last_lines(items.len() + 1)?;
    render_items(&term, &items, &search_term)?;
    cursor.reset_position()?;
    Ok(())
}

fn render_items(
    term: &Term,
    items: &Vec<&str>, 
    search_term: &str) -> std::io::Result<()> {

    term.write_line(&format!("> {}", search_term))?;
    for item in items.iter() {
        if item.find(search_term) != None {
            term.write_line(&format!("{}", item))?;
        }

    }
    Ok(())
   
}
