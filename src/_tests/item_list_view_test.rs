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

mod refresh {
    use super::*;

    #[test]
    fn render_empty_prompt_if_no_items() -> crossterm::Result<()> {
        let mut stdout_spy = StdoutSpy::new();
        let items = Vec::new();
        let mut view = make_10x4_view(&mut stdout_spy);
        let model = make_model(&items);

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
        let mut stdout_spy = StdoutSpy::new();
        let items = vec!["one".into(), "two".into()];
        let mut view = make_10x4_view(&mut stdout_spy);
        let model = make_model(&items);

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
        let mut stdout_spy = StdoutSpy::new();
        let items = vec!["one".into(), "tree".into(), "palm tree".into()];
        let mut view = make_10x4_view(&mut stdout_spy);
        let mut model = make_model(&items);

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
}

mod get_renderable_items_count {
    use super::*;

    #[test]
    fn all_filtered_items_can_be_rendered() {
        let mut stdout_spy = StdoutSpy::new();
        let items = vec!["one".into(), "two".into(), "three".into()];
        let view = make_10x4_view(&mut stdout_spy);
        let model = make_model(&items);

        assert_eq!(view.get_renderable_items_count(&model), 3);
    }

    #[test]
    fn some_filtered_items_can_be_rendered() {
        let mut stdout_spy = StdoutSpy::new();
        let items = vec![
            "1234567890 one".into(),
            "1234567890 two".into(),
            "three".into()];
        let display_width = 10;
        let display_height = 5;
        let view = ItemListView::new(display_width, display_height, &mut stdout_spy);
        let model = make_model(&items);

        // display height = 5 so we can render 2 items with two lines, plus the search prompt
        assert_eq!(view.get_renderable_items_count(&model), 2);
    }
}

fn make_10x4_view(stdout_spy: &mut StdoutSpy) -> ItemListView<StdoutSpy> {
    let display_width = 10;
    let display_height = 4;
    ItemListView::new(display_width, display_height, stdout_spy)
}

fn make_model(items: &Vec<String>) -> ItemListModel {
    let mut model = ItemListModel::new(items);
    model.set_selection_window_height(10);
    model
}
