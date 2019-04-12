use console::{Term};

fn main() -> std::io::Result<()> {
    let term = Term::stdout();
    let mut offset = 1;
    let mut search_term = String::from("");
    term.write_str(&format!("> {}\n", search_term))?;
    render_items(&term, &mut offset)?;


    loop {
        let key = term.read_char().unwrap();
        if key as u8 == 10 {
            return Ok(());
        }
        search_term.push(key);
        term.clear_last_lines(3)?;
        term.write_str(&format!("> {}\n", search_term))?;
        offset = offset + 1;
        render_items(&term, &mut offset)?;
    }
}


fn render_items(term: &Term, offset: &mut u32) -> std::io::Result<()> {
    term.write_str(&format!("Hello {}\n", offset))?;
    term.write_str(&format!("Hello {}\n", *offset+1))?;
    Ok(())
   
}
