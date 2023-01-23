use crate::item_state::ItemState;
use display_me::display;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[display("{} x{}", name, amount)]
pub struct Item {
    pub id: Uuid,
    pub name: String,
    pub amount: usize,
    pub state: ItemState,
}

impl Item {
    pub fn new(name: impl Into<String>, amount: usize) -> Self {
        Item {
            id: Uuid::new_v4(),
            name: name.into(),
            amount,
            state: ItemState::Needed,
        }
    }
}
