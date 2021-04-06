use std::cmp::{max, min};

type FilteredItems<'a> = Vec<&'a String>;
type FilteredItem<'a> = (&'a String, bool);

pub struct FilteredItemsIterator<'a> {
    items: &'a FilteredItems<'a>,
    current_index: u16,
    end_index: u16,
    selected_index: i16,
}

impl<'a> FilteredItemsIterator<'a> {
    pub fn new(items: &'a FilteredItems, start_index: u16, end_index: u16, selected_index: i16) -> FilteredItemsIterator<'a> {
        FilteredItemsIterator {
            items,
            current_index: start_index,
            end_index,
            selected_index,
        }
    }
}

impl<'a> Iterator for FilteredItemsIterator<'a> {
    type Item = FilteredItem<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index == self.end_index as u16 { return None; }
        let result = Some((self.items[self.current_index as usize], self.current_index == self.selected_index as u16));
        self.current_index += 1;
        result
    }
}

pub struct ItemListModel<'a> {
    items: &'a Vec<String>,
    filtered_items: FilteredItems<'a>,
    search_term: String,
    selection: i16,
    selection_window_start: u16,
    selection_window_height: u16,
    selection_window_y: u16,
    max_selection_window_height: u16,
}

impl<'a> ItemListModel<'a> {
    pub fn new(max_selection_window_height: u16, items: &'a Vec<String>) -> ItemListModel<'a> {
        let mut instance = Self {
            items,
            search_term: String::with_capacity(64),
            filtered_items: Vec::with_capacity(10),
            max_selection_window_height,
            selection_window_height: max_selection_window_height,
            selection: 0,
            selection_window_start: 0,
            selection_window_y: 0,
        };
        instance.on_search_term_changed();
        instance
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
        } else if direction == 1 && self.selection_window_y as u16 == self.selection_window_height - 1 {
            self.selection_window_start += 1;
        } else {
            self.selection_window_y = (self.selection_window_y as i16 + direction) as u16;
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

    pub fn get_selected_item(&self) -> &String {
        match self.filtered_items.get(self.selection as usize) {
            Some(item) => { item }
            None => { &self.search_term }
        }
    }

    pub fn get_selection_window_height(&self) -> u16 {
        self.selection_window_height
    }

    pub fn set_selection_window_height(&mut self, value: u16) {
        self.selection_window_height = value;
    }

    pub fn reset_selection_window_height(&mut self) {
        self.set_selection_window_height(self.max_selection_window_height);
    }

    fn on_search_term_changed(&mut self) {
        self.selection = 0;
        self.selection_window_start = 0;
        self.selection_window_y = 0;
        self.filter_items();
    }

    fn filter_items(&mut self) {
        let search_term_upper = self.search_term.to_ascii_uppercase();
        self.filtered_items = self.items.iter()
            .filter(|it| it.to_ascii_uppercase().find(&search_term_upper) != None)
            .collect()
    }

    fn get_selection_window_end(&self) -> u16 {
        return min(
            self.filtered_items.len() as u16,
            self.selection_window_start + self.selection_window_height);
    }
}