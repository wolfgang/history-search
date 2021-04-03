use crate::item_list_model::ItemListModel;

#[test]
fn has_no_filtered_items_after_construction() {
    let items = Vec::new();
    let model = ItemListModel::new((10, 20), &items);
    assert_eq!(model.filtered_items_iter().count(), 0);
    assert_eq!(model.get_selection_window_height(), 10);
}

#[test]
fn if_not_enough_rows_adapt_selection_window_height() {
    let items = Vec::new();
    let model = ItemListModel::new((10, 7), &items);
    assert_eq!(model.get_selection_window_height(), 5);
}

#[test]
fn with_no_search_term_items_are_unfiltered() {
    let items = vec!["item 1".into(), "item 2".into()];
    let mut model = ItemListModel::new((10, 20), &items);
    model.filter_items();
    assert_eq!(get_filtered_items(&model), vec![
        (&"item 1".into(), true),
        (&"item 2".into(), false),
    ]);
}

#[test]
fn add_to_search_term_adds_character_to_search_term() {
    let items = vec!["one".into(), "two".into(), "three ox".into()];
    let mut model = ItemListModel::new((10, 20), &items);

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
    let mut model = ItemListModel::new((10, 20), &items);

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


fn get_filtered_items<'a>(model: &'a ItemListModel) -> Vec<(&'a String, bool)> {
    model.filtered_items_iter().collect()
}