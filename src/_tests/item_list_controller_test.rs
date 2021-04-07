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
        let mut stdout_spy = StdoutSpy::new();
        let mut view = view(&mut stdout_spy);
        let mut model = model(vec!["selected item", "other item"]);

        let mut controller = ItemListController::new(&mut view, &mut model);
        controller.init()?;

        assert!(stdout_spy.written_buf_as_str().contains("> \n\r"));
        stdout_spy.assert_contains("> \n\r");
        stdout_spy.assert_contains(f!("{esc}[7mselected item{esc}"));
        stdout_spy.assert_contains(f!("{esc}[0mother item{esc}"));
        Ok(())
    }
}

fn view(stdout_spy: &mut StdoutSpy) -> ItemListView<StdoutSpy> {
    let display_width = 10;
    let display_height = 5;
    ItemListView::new(display_width, display_height, stdout_spy)
}

fn model(items: Vec<&str>) -> ItemListModel {
    let mut model = ItemListModel::new(items.iter().map(|s| s.to_string()).collect());
    model.set_selection_window_height(9);
    model
}
