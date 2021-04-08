use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
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

mod handle_key_event {
    use super::*;

    #[test]
    fn esc_clears_display() -> crossterm::Result<()> {
        let (mut view, stdout_spy) = view();
        let mut model = model(vec![]);
        let mut controller = controller(&mut view, &mut model);

        controller.handle_key_event(key_event(KeyCode::Esc))?;
        stdout_spy.assert_contains(f!("{esc}[0G          \n\r"));
        stdout_spy.assert_contains(f!("\n\r{esc}[5A{esc}[0G"));
        Ok(())
    }

    #[test]
    fn cursor_up_and_down_moves_selection_() -> crossterm::Result<()> {
        let (mut view, mut stdout_spy) = view();
        let mut model = model(vec!["other item", "selected item"]);
        let mut controller = controller(&mut view, &mut model);

        stdout_spy.clear();
        controller.handle_key_event(key_event(KeyCode::Down))?;
        stdout_spy.assert_contains(f!("\
            {esc}[0mother item{esc}[0m\n\r\
            {esc}[7mselected item"));

        stdout_spy.clear();
        controller.handle_key_event(key_event(KeyCode::Up))?;
        stdout_spy.assert_contains(f!("\
            {esc}[7mother item{esc}[0m\n\r\
            {esc}[0mselected item"));

        Ok(())
    }

    #[test]
    fn character_event_adds_to_search_term() -> crossterm::Result<()> {
        let (mut view, mut stdout_spy) = view();
        let mut model = model(vec!["abcd", "efgh"]);
        let mut controller = controller(&mut view, &mut model);
        stdout_spy.clear();
        controller.handle_key_event(char_event('a'))?;
        controller.handle_key_event(char_event('b'))?;

        stdout_spy.assert_contains("> ab\n\r");
        stdout_spy.assert_contains("abcd");
        stdout_spy.assert_contains_not("efgh");

        Ok(())
    }

    #[test]
    fn backspace_key_removes_from_search_term() -> crossterm::Result<()> {
        let (mut view, mut stdout_spy) = view();
        let mut model = model(vec!["abcd", "efgh"]);
        let mut controller = controller(&mut view, &mut model);
        stdout_spy.clear();
        controller.handle_key_event(char_event('a'))?;
        controller.handle_key_event(key_event(KeyCode::Backspace))?;

        stdout_spy.assert_contains("> \n\r");
        stdout_spy.assert_contains("abcd");
        stdout_spy.assert_contains("efgh");

        Ok(())
    }

    #[test]
    fn verify_infinite_loop_with_empty_list_fixed() -> crossterm::Result<()> {
        let (mut view, mut stdout_spy) = view();
        let mut model = model(vec!["abcd", "efgh"]);
        let mut controller = controller(&mut view, &mut model);
        stdout_spy.clear();
        controller.handle_key_event(char_event('a'))?;
        controller.handle_key_event(char_event('b'))?;

        // Adding x to search term clears list
        stdout_spy.clear();
        controller.handle_key_event(char_event('x'))?;
        stdout_spy.assert_contains_not("abcd");
        stdout_spy.assert_contains_not("efgh");

        // Removing x re-adds 'abcd' entry
        stdout_spy.clear();
        controller.handle_key_event(key_event(KeyCode::Backspace))?;
        stdout_spy.assert_contains("abcd");
        stdout_spy.assert_contains_not("efgh");

        Ok(())
    }


    #[test]
    fn returns_true_except_for_esc_and_enter() {
        let (mut view, _) = view();
        let mut model = model(vec!["abcd", "efgh"]);
        let mut controller = controller(&mut view, &mut model);

        let char_event_result = controller.handle_key_event(key_event(KeyCode::Char('a')));
        let backspace_event_result = controller.handle_key_event(key_event(KeyCode::Backspace));
        let cursor_up_result = controller.handle_key_event(key_event(KeyCode::Up));
        let cursor_down_result = controller.handle_key_event(key_event(KeyCode::Down));
        let esc_result = controller.handle_key_event(key_event(KeyCode::Esc));

        assert_eq!(char_event_result.unwrap(), true);
        assert_eq!(backspace_event_result.unwrap(), true);
        assert_eq!(cursor_up_result.unwrap(), true);
        assert_eq!(cursor_down_result.unwrap(), true);
        assert_eq!(esc_result.unwrap(), false);
    }
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

fn controller<'a>(view: &'a mut ItemListView<StdoutSpy>, model: &'a mut ItemListModel) -> ItemListController<'a, StdoutSpy> {
    let mut controller = ItemListController::new(view, model);
    controller.init().unwrap();
    controller
}

fn char_event(c: char) -> KeyEvent {
    key_event(KeyCode::Char(c))
}

fn key_event(key_code: KeyCode) -> KeyEvent {
    KeyEvent::new(key_code, KeyModifiers::NONE)
}
