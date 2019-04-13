use console::{Key, Term};
use crossterm_cursor::cursor;

fn main() -> std::io::Result<()> {
    let term = Term::stdout();
    let mut offset = 1;
    let mut search_term = String::from("");
    term.write_line(&format!("> {}", search_term))?;
    render_items(&term, "")?;
    let mut cursor = cursor();
    cursor.move_up(3);
    cursor.move_right(2);


    loop {
        let key = term.read_key().unwrap();

        match key {
            Key::Escape => {
                return Ok(());
            }

            Key::Char(ch) => {


                search_term.push(ch);
                cursor.move_down(3);
            
                term.clear_last_lines(3)?;
                term.write_line(&format!("> {}", search_term))?;
                offset = offset + 1;
                render_items(&term, &search_term)?;
                cursor.move_up(3);
                cursor.move_right(search_term.len() as u16 + 2);
            }

            _ => {}
        }
    }
}


fn render_items(term: &Term, search_term: &str) -> std::io::Result<()> {
    term.write_line(&format!("Hello {}", search_term))?;
    term.write_line(&format!("Hello {}", search_term))?;
    Ok(())
   
}
