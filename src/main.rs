use console::{Key, Term};
use crossterm_cursor::cursor;

fn main() -> std::io::Result<()> {
    let term = Term::stdout();
    let mut offset = 1;
    let mut search_term = String::from("");
    term.write_line(&format!("> {}", search_term))?;
    render_items(&term, &mut offset)?;
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
                render_items(&term, &mut offset)?;
                cursor.move_up(3);
                cursor.move_right(offset as u16 + 1);
            }

            _ => {}
        }
    }
}


fn render_items(term: &Term, offset: &mut u32) -> std::io::Result<()> {
    term.write_line(&format!("Hello {}", offset))?;
    term.write_line(&format!("Hello {}", *offset+1))?;
    Ok(())
   
}
