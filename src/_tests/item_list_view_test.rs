use crate::item_list_view::ItemListView;

struct StdoutSpy {}

#[test]
fn reset_cursor_column() -> crossterm::Result<()> {
    let stdout_spy = StdoutSpy {};
    // let mut view = ItemListView::new(&stdout_spy);
    // view.reset_cursor_column()?;
    Ok(())
}