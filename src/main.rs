use console::{Key, Term};
use crossterm_cursor::cursor;

fn main() -> std::io::Result<()> {
    let term = Term::stdout();
    let mut search_term = String::from("");
    
    render_items(&term, &search_term)?;
    let mut cursor = cursor();
    cursor.move_up(3);
    cursor.move_right(2);


    loop {
        let key = term.read_key().unwrap();

        match key {
            Key::Escape => { return Ok(()); }

            Key::Char(ch) => {
                search_term.push(ch);
                cursor.save_position()?;
                cursor.move_down(3);
            
                term.clear_last_lines(3)?;
                render_items(&term, &search_term)?;
                cursor.reset_position()?;
                cursor.move_right(1);
            }

            _ => {}
        }
    }
}


fn render_items(term: &Term, search_term: &str) -> std::io::Result<()> {
    term.write_line(&format!("> {}", search_term))?;
    term.write_line(&format!("Hello {}", search_term))?;
    term.write_line(&format!("Hello {}", search_term))?;
    Ok(())
   
}
