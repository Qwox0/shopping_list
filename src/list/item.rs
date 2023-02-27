use crate::{language::text_macro::text, list::List, util::from_with_scope::FromWithScope};
use display_me::display;
use leptos::*;
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::{query::Query, sqlite::SqliteArguments, QueryBuilder, Sqlite};
use uuid::Uuid;

#[component]
pub fn ItemView(cx: Scope, item: Item) -> impl IntoView {
    let mut list: List = use_context(cx).expect("`List` is available");

    let delete = move |_e| {
        let remove_question = text!(cx, |d| format!(
            "{}{:?}{}",
            d.item.remove_question_1,
            item.name.get(),
            d.item.remove_question_2
        ))();
        let do_remove = window()
            .confirm_with_message(&remove_question)
            .unwrap_or_else(|jsVal| {
                log!("remove error {:?}", jsVal);
                false
            });
        if do_remove {
            log!("removing item");
            list.remove(item.id);
        } else {
            log!("not removing item");
        }
    };

    view! { cx,
        <div class="item" state=move || item.state.get().as_attribute() >
            <input type="checkbox"
                checked=move || item.state.with(|s| s.is_done())
                on:change=move |e| item.state.set(event_target_checked(&e).into())
            />
            <div class="text">
                {move || { format!("{}", item) }}
            </div>
            <div class="buttons">
                <input type="image"
                    alt=text!(cx, |d| d.item.edit.clone())
                    src="/img/pen.webp"
                    on:click=move |_e| {}
                    />
                <input type="image"
                    alt=text!(cx, |d| d.item.remove.clone())
                    src="/img/bin.webp"
                    on:click=delete
                />
            </div>
        </div>

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
    pub fn from_new_item(cx: Scope, new_item: Item) -> Self {
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
}

// ------ Serialized -------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
#[display("{} x{}", name, amount)]
pub struct ItemSerialized {
    id: Uuid,
    pub name: String,
    pub amount: u32,
    pub state: ItemState,
}

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

impl ItemSerialized {
    pub fn new(name: impl Into<String>, amount: u32, state: ItemState) -> Self {
        ItemSerialized {
            id: Uuid::new_v4(),
            name: name.into(),
            amount,
            state,
        }
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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
