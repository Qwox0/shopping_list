use super::item::Item;

#[derive(Clone)]
pub struct ItemList {
    pub items: Vec<Item>,
}

impl ItemList {
    pub fn new() -> Self {
        ItemList { items: Vec::new() }
    }
}
