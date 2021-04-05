use fstrings::{f, format_args_f};

use crate::_tests::stdout_spy::StdoutSpy;
use crate::item_list_model::ItemListModel;
use crate::item_list_view::ItemListView;

#[allow(non_upper_case_globals)]
const esc: &str = "\u{1b}";

#[test]
fn reset_cursor_column_writes_correct_escape_sequence() -> crossterm::Result<()> {
    let mut stdout_spy = StdoutSpy::new();
    let mut view = ItemListView::new(&mut stdout_spy);
    view.reset_cursor_column()?;
    stdout_spy.assert(f!("{esc}[0G"));
    Ok(())
}

mod render {
    use super::*;

    #[test]
    fn render_with_no_items() -> crossterm::Result<()> {
        let mut stdout_spy = StdoutSpy::new();
        let mut view = ItemListView::new(&mut stdout_spy);
        let items = Vec::new();
        let model = ItemListModel::new(10, &items);
        view.render(&model)?;
        stdout_spy.assert(f!("{esc}[0G{esc}7> \n\r{esc}8{esc}[3G"));
        Ok(())
    }
}