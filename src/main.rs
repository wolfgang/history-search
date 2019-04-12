use console::{Key, Term};

fn main() -> std::io::Result<()> {
    let term = Term::stdout();
    let mut offset = 1;
    render_items(&term, &mut offset)?;

    loop {
        match term.read_key()? {
            Key::Enter => {
                return Ok(());
            }
            _ => {
                offset = offset + 1;
                term.clear_last_lines(3)?;
                render_items(&term, &mut offset)?;
            }
        }
    }
}


fn render_items(term: &Term, offset: &mut u32) -> std::io::Result<()> {
    term.write_str("> ... search goes here ...\n");
    term.write_str(&format!("Hello {}\n", offset))?;
    term.write_str(&format!("Hello {}\n", *offset+1))?;
    Ok(())
   
}
