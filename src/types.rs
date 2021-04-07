use std::cell::RefCell;
use std::io::Write;
use std::rc::Rc;

pub type FilteredItems = Vec<String>;
pub type SelectableItem = (String, bool);
pub type StdoutRef = Rc<RefCell<dyn Write>>;
