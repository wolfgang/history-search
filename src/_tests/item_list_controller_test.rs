use fstrings::{f, format_args_f};

use crate::_tests::stdout_spy::{StdoutSpy, StdoutSpyRef};
use crate::item_list_controller::ItemListController;
use crate::item_list_model::ItemListModel;
use crate::item_list_view::ItemListView;

#[allow(non_upper_case_globals)]
const esc: &str = "\u{1b}";

mod initialisation {
    use super::*;

    #[test]
    fn init_renders_view() -> crossterm::Result<()> {
        let (mut view, stdout_spy) = view();
        let mut model = model(vec!["selected item", "other item"]);

        let mut controller = ItemListController::new(&mut view, &mut model);
        controller.init()?;

        stdout_spy.assert_contains("> \n\r");
        stdout_spy.assert_contains(f!("{esc}[7mselected item{esc}"));
        stdout_spy.assert_contains(f!("{esc}[0mother item{esc}"));
        Ok(())
    }
}

mod key_events {
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    use super::*;

    #[test]
    fn esc_clears_and_returns_false() -> crossterm::Result<()> {
        let (mut view, stdout_spy) = view();
        let mut model = model(vec![]);
        let mut controller = controller(&mut view, &mut model);

        let result = controller.handle_key_event(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
        assert_eq!(result.unwrap(), false);
        stdout_spy.assert_contains(f!("{esc}[0G          \n\r"));
        stdout_spy.assert_contains(f!("\n\r{esc}[5A{esc}[0G"));
        Ok(())
    }

    #[test]
    fn cursor_down_moves_selection_down() -> crossterm::Result<()> {
        let (mut view, stdout_spy) = view();
        let mut model = model(vec!["other item", "selected item"]);
        let mut controller = controller(&mut view, &mut model);

        let result = controller.handle_key_event(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
        assert_eq!(result.unwrap(), true);
        stdout_spy.assert_contains(f!("\
            {esc}[0mother item{esc}[0m\n\r\
            {esc}[7mselected item"));

        Ok(())
    }
}

fn controller<'a>(view: &'a mut ItemListView<StdoutSpy>, model: &'a mut ItemListModel) -> ItemListController<'a, StdoutSpy> {
    let mut controller = ItemListController::new(view, model);
    controller.init().unwrap();
    controller
}

fn view() -> (ItemListView<StdoutSpy>, StdoutSpyRef) {
    let display_width = 10;
    let display_height = 5;
    let stdout_spy = StdoutSpyRef::new();
    (ItemListView::new(display_width, display_height, stdout_spy.clone()), stdout_spy)
}

fn model(items: Vec<&str>) -> ItemListModel {
    let mut model = ItemListModel::new(items.iter().map(|s| s.to_string()).collect());
    model.set_selection_window_height(9);
    model
}
