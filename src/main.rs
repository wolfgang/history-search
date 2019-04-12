use console::{Key, Term};

fn main() -> std::io::Result<()> {
    let term = Term::stdout();
    let mut offset = 1;
    let mut search_term = String::from("");
    term.write_str(&format!("> {}\n", search_term))?;
    render_items(&term, &mut offset)?;


    loop {
        let key = term.read_key().unwrap();

        match key {
            Key::Escape => {
                return Ok(());
            }

            Key::Char(ch) => {
                search_term.push(ch);
                term.clear_last_lines(3)?;
                term.write_str(&format!("> {}\n", search_term))?;
                offset = offset + 1;
                render_items(&term, &mut offset)?;                
            }

            _ => {}
        }
    }
}


fn render_items(term: &Term, offset: &mut u32) -> std::io::Result<()> {
    term.write_str(&format!("Hello {}", offset))?;
    term.write_str(&format!("Hello {}", *offset+1))?;
    Ok(())
   
}
