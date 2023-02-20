pub mod item_view;

use crate::language::text_macro::text;
use display_me::display;
use item_view::{ItemView, ItemViewProps};
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
    pub fn with_state(cx: Scope, name: impl Into<String>, amount: usize, state: ItemState) -> Self {
        Item {
            id: Uuid::new_v4(),
            name: create_rw_signal(cx, name.into()),
            amount: create_rw_signal(cx, amount),
            state: create_rw_signal(cx, state),
        }
    }

    pub fn new(cx: Scope, name: impl Into<String>, amount: usize) -> Self {
        Item::with_state(cx, name, amount, ItemState::Needed)
    }

    /// Clone Item from new_item and decouple Signals.
    pub fn from_new_item(cx: Scope, new_item: Item) -> Self {
        Item::with_state(
            cx,
            new_item.name.get(),
            new_item.amount.get(),
            new_item.state.get(),
        )
    }

    pub fn from_serialized(cx: Scope, serialized: ItemSerialized) -> Self {
        Item {
            id: serialized.id,
            name: create_rw_signal(cx, serialized.name),
            amount: create_rw_signal(cx, serialized.amount),
            state: create_rw_signal(cx, serialized.state),
        }
    }
}

// ------ Serialized -------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[display("{} x{}", name, amount)]
pub struct ItemSerialized {
    pub id: Uuid,
    pub name: String,
    pub amount: usize,
    pub state: ItemState,
}

impl From<&Item> for ItemSerialized {
    fn from(value: &Item) -> Self {
        ItemSerialized {
            id: value.id,
            name: value.name.get(),
            amount: value.amount.get(),
            state: value.state.get(),
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
