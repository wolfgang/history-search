use std::io::{Error, Write};
use std::str::from_utf8;

use fstrings::{f, format_args_f};

use crate::item_list_view::ItemListView;

struct StdoutSpy {
    pub written_buf: Vec<u8>,
}

impl StdoutSpy {
    pub fn new() -> Self {
        Self {
            written_buf: Vec::with_capacity(256),
        }
    }

    pub fn written_buf_as_str(&self) -> &str {
        from_utf8(self.written_buf.as_slice()).unwrap()
    }
}

impl Write for StdoutSpy {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        let mut new_vec = Vec::from(buf);
        self.written_buf.append(&mut new_vec);
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), Error> {
        Ok(())
    }
}

#[test]
fn fstring() {
    let greetings = "hello";
    assert_eq!(f!("{greetings} there"), "hello there");
}

#[test]
fn reset_cursor_column_writes_correct_escape_sequence() -> crossterm::Result<()> {
    let mut stdout_spy = StdoutSpy::new();
    let mut view = ItemListView::new(&mut stdout_spy);
    view.reset_cursor_column()?;
    assert_eq!(stdout_spy.written_buf_as_str(), "\u{1b}[0G");
    Ok(())
}

mod render {
    use crate::item_list_model::ItemListModel;

    use super::*;

    #[test]
    fn render_with_no_items() -> crossterm::Result<()> {
        let mut stdout_spy = StdoutSpy::new();
        let mut view = ItemListView::new(&mut stdout_spy);
        let items = Vec::new();
        let model = ItemListModel::new(10, &items);
        view.render(&model)?;
        assert_eq!(stdout_spy.written_buf_as_str(), "\u{1b}[0G\u{1b}7> \n\r\u{1b}8\u{1b}[3G");
        Ok(())
    }
}