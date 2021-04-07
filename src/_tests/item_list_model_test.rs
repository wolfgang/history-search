use crate::item_list_model::ItemListModel;

mod construction {
    use super::*;

    #[test]
    fn has_no_filtered_items_after_construction() {
        let items = Vec::new();
        let model = ItemListModel::new(&items);
        assert_eq!(get_filtered_items(&model), vec![]);
        assert_eq!(model.get_selection_window_height(), 0)
    }

    #[test]
    fn set_selection_window_height_sets_you_know_what() {
        let items = Vec::new();
        let mut model = ItemListModel::new(&items);
        model.set_selection_window_height(12);
        assert_eq!(model.get_selection_window_height(), 12);
    }
}

mod filtering {
    use super::*;

    #[test]
    fn with_no_search_term_items_are_unfiltered() {
        let items = vec!["item 1".into(), "item 2".into()];
        let model = model(&items);
        assert_eq!(get_filtered_items(&model), vec![
            (&"item 1".into(), true),
            (&"item 2".into(), false),
        ]);
    }

    #[test]
    fn add_to_search_term_adds_character_to_search_term() {
        let items = vec!["one".into(), "two".into(), "three ox".into()];
        let mut model = model(&items);

        model.add_to_search_term('o');
        assert_eq!(get_filtered_items(&model), vec![
            (&"one".into(), true),
            (&"two".into(), false),
            (&"three ox".into(), false)
        ]);

        model.add_to_search_term('x');
        assert_eq!(get_filtered_items(&model), vec![
            (&"three ox".into(), true)
        ]);
    }

    #[test]
    fn pop_search_term_removes_character_from_search_term() {
        let items = vec!["12".into(), "13".into(), "14".into()];
        let mut model = model(&items);

        model.add_to_search_term('1');
        model.add_to_search_term('3');
        assert_eq!(get_filtered_items(&model), vec![
            (&"13".into(), true)]);

        model.pop_search_term();
        assert_eq!(get_filtered_items(&model), vec![
            (&"12".into(), true),
            (&"13".into(), false),
            (&"14".into(), false)]);
    }
}

mod selection {
    use super::*;

    #[test]
    fn change_selection_moves_selection() {
        let items = vec!["one".into(), "two".into()];
        let mut model = model(&items);

        model.change_selection(1);
        assert_eq!(get_filtered_items(&model), vec![
            (&"one".into(), false),
            (&"two".into(), true),
        ]);
        model.change_selection(-1);
        assert_eq!(get_filtered_items(&model), vec![
            (&"one".into(), true),
            (&"two".into(), false),
        ])
    }

    #[test]
    fn change_selection_is_constrained() {
        let items = vec!["one".into(), "two".into()];
        let mut model = model(&items);

        change_selection_times(5, 1, &mut model);
        assert_eq!(get_filtered_items(&model), vec![
            (&"one".into(), false),
            (&"two".into(), true),
        ]);
        change_selection_times(10, -1, &mut model);
        assert_eq!(get_filtered_items(&model), vec![
            (&"one".into(), true),
            (&"two".into(), false),
        ])
    }

    #[test]
    fn change_selection_scrolls() {
        let items = vec![
            "one".into(),
            "two".into(),
            "three".into(),
            "four".into(),
        ];

        let mut model = model(&items);
        model.set_selection_window_height(3);

        assert_eq!(model.get_selection_window_height(), 3);
        model.change_selection(1);
        assert_eq!(get_filtered_items(&model), vec![
            (&"one".into(), false),
            (&"two".into(), true),
            (&"three".into(), false),
        ]);

        model.change_selection(1);
        assert_eq!(get_filtered_items(&model), vec![
            (&"one".into(), false),
            (&"two".into(), false),
            (&"three".into(), true),
        ]);


        change_selection_times(5, 1, &mut model);
        assert_eq!(get_filtered_items(&model), vec![
            (&"two".into(), false),
            (&"three".into(), false),
            (&"four".into(), true),
        ]);

        model.change_selection(-1);
        assert_eq!(get_filtered_items(&model), vec![
            (&"two".into(), false),
            (&"three".into(), true),
            (&"four".into(), false),
        ]);
        model.change_selection(-1);
        assert_eq!(get_filtered_items(&model), vec![
            (&"two".into(), true),
            (&"three".into(), false),
            (&"four".into(), false),
        ]);

        change_selection_times(5, -1, &mut model);
        assert_eq!(get_filtered_items(&model), vec![
            (&"one".into(), true),
            (&"two".into(), false),
            (&"three".into(), false),
        ])
    }

    #[test]
    fn change_selection_returns_false_if_selection_is_unchanged() {
        let items = vec!["one".into(), "two".into()];
        let mut model = model(&items);

        assert!(!model.change_selection(-1));
        assert!(model.change_selection(1));
        assert!(!model.change_selection(1));
        assert!(model.change_selection(-1));
    }
}

mod get_selected_item {
    use super::*;

    #[test]
    fn get_selected_item_returns_currently_selected_item() {
        let items = vec!["one".into(), "two".into()];
        let mut model = model(&items);

        assert_eq!(model.get_selected_item(), "one");
        model.change_selection(1);
        assert_eq!(model.get_selected_item(), "two");
    }

    #[test]
    fn search_term_narrows_selection() {
        let items = vec!["one".into(), "two".into()];
        let mut model = model(&items);

        model.add_to_search_term('t');
        assert_eq!(model.get_selected_item(), "two");
    }

    #[test]
    fn get_selected_item_returns_search_term_if_no_match() {
        let items = vec!["one".into(), "two".into()];
        let mut model = model(&items);

        model.add_to_search_term('x');
        model.add_to_search_term('y');
        assert_eq!(model.get_selected_item(), "xy");
    }
}

fn model(items: &Vec<String>) -> ItemListModel {
    let mut model = ItemListModel::new(items);
    model.set_selection_window_height(10);
    model
}

fn get_filtered_items<'a>(model: &'a ItemListModel) -> Vec<(&'a String, bool)> {
    model.selectable_items_iter().collect()
}

fn change_selection_times(times: u16, direction: i16, model: &mut ItemListModel) {
    for _ in 0..times { model.change_selection(direction); }
}