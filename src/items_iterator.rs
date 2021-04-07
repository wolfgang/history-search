use crate::types::{FilteredItems, SelectableItem};

pub struct ItemsIterator<'a> {
    items: &'a FilteredItems,
    current_index: u16,
    end_index: u16,
    selected_index: i16,
}

impl<'a> ItemsIterator<'a> {
    pub fn new(items: &'a FilteredItems, start_index: u16, end_index: u16, selected_index: i16) -> ItemsIterator<'a> {
        ItemsIterator {
            items,
            current_index: start_index,
            end_index,
            selected_index,
        }
    }
}

impl Iterator for ItemsIterator<'_> {
    type Item = SelectableItem;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index == self.end_index as u16 { return None; }
        let result = Some((self.items[self.current_index as usize].to_string(), self.current_index == self.selected_index as u16));
        self.current_index += 1;
        result
    }
}
