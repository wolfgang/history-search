use std::cmp::{max, min};

type FilteredItems<'a> = Vec<&'a String>;
type FilteredItem<'a> = (&'a String, bool);

pub struct ItemListModel<'a> {
    items: &'a Vec<String>,
    filtered_items: FilteredItems<'a>,
    search_term: String,
    selection: i16,
    selection_window_start: i16,
    selection_window_height: i16,
    selection_window_y: i16,
}

pub struct FilteredItemsIterator<'a> {
    items: &'a FilteredItems<'a>,
    end_index: i16,
    selected_index: i16,
    current_index: i16,
}

impl<'a> FilteredItemsIterator<'a> {
    pub fn new(items: &'a FilteredItems, start_index: i16, end_index: i16, selected_index: i16) -> FilteredItemsIterator<'a> {
        FilteredItemsIterator {
            items,
            end_index,
            selected_index,
            current_index: start_index,
        }
    }
}

impl<'a> Iterator for FilteredItemsIterator<'a> {
    type Item = FilteredItem<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index == self.end_index { return None; }
        let result = Some((self.items[self.current_index as usize], self.current_index == self.selected_index));
        self.current_index += 1;
        result
    }
}

impl<'a> ItemListModel<'a> {
    pub fn new(display_size: (u16, u16), items: &'a Vec<String>) -> ItemListModel<'a> {
        let (_, rows) = display_size;
        let selection_window_height = min(rows as i16 - 2, 10);
        Self {
            items,
            search_term: String::with_capacity(64),
            filtered_items: Vec::with_capacity(10),
            selection_window_height,
            selection: 0,
            selection_window_start: 0,
            selection_window_y: 0,

        }
    }

    pub fn add_to_search_term(&mut self, ch: char) {
        self.search_term.push(ch);
        self.on_search_term_changed();
    }

    pub fn pop_search_term(&mut self) -> bool {
        if !self.search_term.is_empty() {
            self.search_term.pop();
            self.on_search_term_changed();
            return true;
        }
        false
    }

    pub fn get_search_term(&self) -> &String {
        return &self.search_term;
    }

    pub fn change_selection(&mut self, direction: i16) -> bool {
        let num_items = self.filtered_items.len() as i16;
        let prev_selection = self.selection;
        self.selection = max(0, min(num_items - 1, self.selection + direction));
        if prev_selection == self.selection { return false; }

        if direction == -1 && self.selection_window_y == 0 {
            self.selection_window_start -= 1;
        } else if direction == 1 && self.selection_window_y == self.selection_window_height - 1 {
            self.selection_window_start += 1;
        } else {
            self.selection_window_y += direction;
        }
        true
    }

    pub fn filtered_items_iter(&self) -> FilteredItemsIterator {
        FilteredItemsIterator::new(
            &self.filtered_items,
            self.selection_window_start,
            self.get_selection_window_end(),
            self.selection)
    }

    pub fn filter_items(&mut self) {
        let search_term_upper = self.search_term.to_ascii_uppercase();
        self.filtered_items = self.items.iter()
            .filter(|it| it.to_ascii_uppercase().find(&search_term_upper) != None)
            .collect()
    }

    pub fn get_selected_item(&self) -> &String {
        match self.filtered_items.get(self.selection as usize) {
            Some(item) => { item }
            None => { &self.search_term }
        }
    }

    pub fn get_selection_window_height(&self) -> i16 {
        self.selection_window_height
    }

    fn on_search_term_changed(&mut self) {
        self.selection = 0;
        self.selection_window_start = 0;
        self.selection_window_y = 0;
        self.filter_items();
    }

    fn get_selection_window_end(&self) -> i16 {
        return min(
            self.filtered_items.len() as i16,
            self.selection_window_start + self.selection_window_height);
    }
}