use fstrings::{f, format_args_f};

use crate::_tests::stdout_spy::{StdoutSpy, StdoutSpyRef};
use crate::item_list_model::ItemListModel;
use crate::item_list_view::ItemListView;

#[allow(non_upper_case_globals)]
const esc: &str = "\u{1b}";

#[test]
fn reset_cursor_column_writes_correct_escape_sequence() -> crossterm::Result<()> {
    let stdout_spy = StdoutSpyRef::new();
    let mut view = ItemListView::new(10, 10, stdout_spy.clone());
    view.reset_cursor_column()?;
    stdout_spy.assert(f!("{esc}[0G"));
    Ok(())
}


mod refresh {
    use super::*;

    #[test]
    fn render_empty_prompt_if_no_items() -> crossterm::Result<()> {
        let (mut view, stdout_spy) = make_10x4_view();
        let model = make_model(Vec::new());

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

    #[test]
    fn render_all_items_if_no_search_term() -> crossterm::Result<()> {
        let (mut view, stdout_spy) = make_10x4_view();
        let model = make_model(vec!["one".into(), "two".into()]);

        view.refresh(&model)?;
        let clear = f!("{esc}[0G          \n\r          \n\r          \n\r          \n\r{esc}[4A");
        let render_prompt = f!("{esc}[0G{esc}7> \n\r");
        let restore_cursor = f!("{esc}8{esc}[3G");
        stdout_spy.assert(f!("\
        {clear}\
        {render_prompt}\
        {esc}[7mone{esc}[0m\n\r\
        {esc}[0mtwo{esc}[0m\n\r\
        {restore_cursor}"));
        Ok(())
    }

    #[test]
    fn render_search_term_and_matching_items() -> crossterm::Result<()> {
        let (mut view, stdout_spy) = make_10x4_view();
        let mut model = make_model(vec!["one".into(), "tree".into(), "palm tree".into()]);

        model.add_to_search_term('t');
        model.add_to_search_term('r');
        model.add_to_search_term('e');

        view.refresh(&model)?;
        let clear = f!("{esc}[0G          \n\r          \n\r          \n\r          \n\r{esc}[4A");
        let render_prompt = f!("{esc}[0G{esc}7> tre\n\r");
        let restore_cursor = f!("{esc}8{esc}[6G");
        stdout_spy.assert(f!("\
        {clear}\
        {render_prompt}\
        {esc}[7mtree{esc}[0m\n\r\
        {esc}[0mpalm tree{esc}[0m\n\r\
        {restore_cursor}"));
        Ok(())
    }

    #[test]
    fn cut_off_longer_lines() -> crossterm::Result<()> {
        let (mut view, stdout_spy) = make_10x4_view();
        let model = make_model(vec![
            "line that is longer than the display width".into(),
            "line two".into()
        ]);

        view.refresh(&model)?;
        stdout_spy.assert_contains(f!("\
        {esc}[7mline that {esc}[0m\n\r\
        {esc}[0mline two{esc}[0m\n\r\
        "));
        Ok(())
    }
}

fn make_10x4_view() -> (ItemListView<StdoutSpy>, StdoutSpyRef) {
    let display_width = 10;
    let display_height = 4;
    let stdout_spy = StdoutSpyRef::new();
    (ItemListView::new(display_width, display_height, stdout_spy.clone()), stdout_spy)
}

fn make_model(items: Vec<String>) -> ItemListModel {
    let mut model = ItemListModel::new(items);
    model.set_selection_window_height(10);
    model
}
