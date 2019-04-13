use console::{Key, Term};
use crossterm_cursor::cursor;

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
                search_term.push(ch);
                cursor.save_position()?;
                cursor.move_down(items.len() as u16 + 1);
            
                term.clear_last_lines(items.len() + 1)?;
                render_items(&term, &items, &search_term)?;
                cursor.reset_position()?;
                cursor.move_right(1);
            }

            _ => {}
        }
    }
}

fn render_items(
    term: &Term,
    items: &Vec<&str>, 
    search_term: &str) -> std::io::Result<()> {

    term.write_line(&format!("> {}", search_term))?;
    for item in items.iter() {
        term.write_line(&format!("{} {}", item, search_term))?;

    }
    Ok(())
   
}
