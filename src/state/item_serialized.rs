use crate::state::item::{Item, ItemState};
use display_me::display;
use leptos::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
#[display("{} x{}", name, amount)]
pub struct ItemSerialized {
    pub id: Uuid,
    pub name: String,
    pub amount: u32,
    pub state: ItemState,
}

/*
#[cfg(feature = "ssr")]
impl crate::db::InDb for ItemSerialized {
    const COLUMNS_TUPLE: &'static str = "(id, name, amount, state)";

    fn bind_values<'a>(
        &self,
        query_builder: &'a mut QueryBuilder<'a, Sqlite>,
    ) -> &mut QueryBuilder<'a, Sqlite> {
        query_builder
            .push_bind(self.id.clone())
            .push_bind(self.name.clone())
            .push_bind(self.amount.clone())
            .push_bind(self.state.clone())
    }
}
*/

impl ItemSerialized {
    pub fn new(name: impl Into<String>, amount: u32, state: ItemState) -> Self {
        ItemSerialized {
            id: Uuid::new_v4(),
            name: name.into(),
            amount,
            state,
        }
    }

    pub fn new_with_item(item: &Item) -> Self {
        Self::new(item.name.get(), item.amount.get(), item.state.get())
    }
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
