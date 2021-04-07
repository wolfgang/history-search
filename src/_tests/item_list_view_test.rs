use fstrings::{f, format_args_f};

use crate::_tests::stdout_spy::StdoutSpy;
use crate::item_list_model::ItemListModel;
use crate::item_list_view::ItemListView;

#[allow(non_upper_case_globals)]
const esc: &str = "\u{1b}";

#[test]
fn reset_cursor_column_writes_correct_escape_sequence() -> crossterm::Result<()> {
    let mut stdout_spy = StdoutSpy::new();
    let mut view = ItemListView::new(10, 10, &mut stdout_spy);
    view.reset_cursor_column()?;
    stdout_spy.assert(f!("{esc}[0G"));
    Ok(())
}

mod render {
    use super::*;

    #[test]
    fn render_empty_prompt_if_no_items() -> crossterm::Result<()> {
        let mut stdout_spy = StdoutSpy::new();
        let mut view = ItemListView::new(10, 10, &mut stdout_spy);
        let items = Vec::new();
        let model = ItemListModel::new(&items);
        view.render(&model)?;
        stdout_spy.assert(f!("{esc}[0G{esc}7> \n\r{esc}8{esc}[3G"));
        Ok(())
    }

    #[test]
    fn render_all_items_if_no_search_term() -> crossterm::Result<()> {
        let mut stdout_spy = StdoutSpy::new();
        let mut view = ItemListView::new(10, 10, &mut stdout_spy);
        let items = vec!["one".into(), "two".into()];
        let mut model = ItemListModel::new(&items);
        model.set_selection_window_height(10);

        view.render(&model)?;
        stdout_spy.assert(f!("\
        {esc}[0G{esc}7> \n\r\
        {esc}[7mone{esc}[0m\n\r\
        {esc}[0mtwo{esc}[0m\n\r\
        {esc}8{esc}[3G"));
        Ok(())
    }

    #[test]
    fn render_search_term_and_matching_items() -> crossterm::Result<()> {
        let mut stdout_spy = StdoutSpy::new();
        let mut view = ItemListView::new(10, 10, &mut stdout_spy);
        let items = vec!["one".into(), "tree".into(), "palm tree".into()];
        let mut model = ItemListModel::new(&items);
        model.set_selection_window_height(10);

        model.add_to_search_term('t');
        model.add_to_search_term('r');
        model.add_to_search_term('e');
        view.render(&model)?;
        stdout_spy.assert(f!("\
        {esc}[0G{esc}7> tre\n\r\
        {esc}[7mtree{esc}[0m\n\r\
        {esc}[0mpalm tree{esc}[0m\n\r\
        {esc}8{esc}[6G"));
        Ok(())
    }
}

mod refresh {
    use super::*;

    #[test]
    fn render_empty_prompt_if_no_items() -> crossterm::Result<()> {
        let mut stdout_spy = StdoutSpy::new();
        let display_height = 4;
        let display_width = 10;
        let mut view = ItemListView::new(display_height, display_width, &mut stdout_spy);
        let items = Vec::new();
        let model = ItemListModel::new(&items);
        view.refresh(&model)?;
        let clear = f!("{esc}[0G          \n\r          \n\r          \n\r          \n\r{esc}[4A");
        let render_prompt = f!("{esc}[0G{esc}7> \n\r");
        let restore_cursor = f!("{esc}8{esc}[3G");
        stdout_spy.assert(f!("\
        {clear}\
        {render_prompt}\
        {restore_cursor}"));
        Ok(())
    }
}