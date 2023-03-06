use crate::state::item_serialized::ItemSerialized;
use display_me::display;
use leptos::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
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

#[derive(Debug, Clone)]
#[display("{} x{}", name(), amount())]
pub struct Item {
    pub id: Uuid,
    pub name: RwSignal<String>,
    pub amount: RwSignal<u32>,
    pub state: RwSignal<ItemState>,
}

impl Item {
    pub fn new(cx: Scope, name: impl Into<String>, amount: u32, state: ItemState) -> Self {
        Item {
            id: Uuid::new_v4(),
            name: create_rw_signal(cx, name.into()),
            amount: create_rw_signal(cx, amount),
            state: create_rw_signal(cx, state),
        }
    }

    pub fn empty(cx: Scope) -> Self {
        Item::new(cx, "", 1, ItemState::Needed)
    }

    /// Clone Item from new_item and decouple Signals.
    pub fn from_new_item(cx: Scope, new_item: &Item) -> Self {
        Item::new(
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

    pub fn from_serialized_ref(cx: Scope, serialized: &ItemSerialized) -> Self {
        Item {
            id: serialized.id,
            name: create_rw_signal(cx, serialized.name.clone()),
            amount: create_rw_signal(cx, serialized.amount.clone()),
            state: create_rw_signal(cx, serialized.state.clone()),
        }
    }
}

// ------ Serialized -------
