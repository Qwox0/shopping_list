use display_me::display;
use leptos::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone)]
#[display("{} x{}", name(), amount())]
pub struct Item {
    pub id: Uuid,
    pub name: RwSignal<String>,
    pub amount: RwSignal<usize>,
    pub state: RwSignal<ItemState>,
}

impl Item {
    pub fn new(cx: Scope, name: impl Into<String>, amount: usize) -> Self {
        Item {
            id: Uuid::new_v4(),
            name: create_rw_signal(cx, name.into()),
            amount: create_rw_signal(cx, amount),
            state: create_rw_signal(cx, ItemState::Needed),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ItemState {
    Needed,
    Completed,
}

impl ItemState {
    pub fn is_done(&self) -> bool {
        match self {
            ItemState::Completed => true,
            _ => false,
        }
    }

    pub fn as_attribute(&self) -> String {
        match self {
            ItemState::Completed => "completed",
            ItemState::Needed => "needed",
        }
        .to_string()
    }
}

impl From<bool> for ItemState {
    fn from(value: bool) -> Self {
        if value {
            ItemState::Completed
        } else {
            ItemState::Needed
        }
    }
}

/*
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemSerialized {
    pub id: Uuid,
    pub name: String,
    pub amount: usize,
    pub state: ItemState,
}

impl From<&Item> for ItemSerialized {
    fn from(item: &Item) -> Self {
        ItemSerialized {
            id: item.id,
            name: item.name.get(),
            amount: item.amount.get(),
            state: item.state.get(),
        }
    }
}

impl ItemSerialized {
    fn into_item(self, cx: Scope) -> Item {
        Item {
            id: self.id,
            name: create_rw_signal(cx, self.name),
            amount: create_rw_signal(cx, self.amount),
            state: create_rw_signal(cx, self.state),
        }
    }
}
*/
