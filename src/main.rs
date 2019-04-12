use std::thread;
use std::time::Duration;

use console::Term;


fn main() -> std::io::Result<()> {
    let term = Term::stdout();
    term.move_cursor_down(10)?;
    term.write_line("Hello 1")?;
    term.move_cursor_up(5)?;
    term.write_line("Hello 2")?;
    Ok(())
}
