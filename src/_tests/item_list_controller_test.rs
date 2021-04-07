use fstrings::{f, format_args_f};

use crate::_tests::stdout_spy::StdoutSpy;
use crate::item_list_controller::ItemListController;
use crate::item_list_model::ItemListModel;
use crate::item_list_view::ItemListView;

#[allow(non_upper_case_globals)]
const esc: &str = "\u{1b}";

mod initialisation {
    use super::*;

    #[test]
    fn init_renders_view() -> crossterm::Result<()> {
        let display_width = 10;
        let display_height = 5;
        let mut stdout_spy = StdoutSpy::new();
        let mut view = ItemListView::new(display_width, display_height, &mut stdout_spy);
        let mut model = ItemListModel::new(vec!["selected item".into(), "other item".into()]);
        model.set_selection_window_height(9);

        let mut controller = ItemListController::new(&mut view, &mut model);
        controller.init()?;

        assert!(stdout_spy.written_buf_as_str().contains("> \n\r"));
        stdout_spy.assert_contains("> \n\r");
        stdout_spy.assert_contains(f!("{esc}[7mselected item{esc}"));
        stdout_spy.assert_contains(f!("{esc}[0mother item{esc}"));
        Ok(())
    }
}
