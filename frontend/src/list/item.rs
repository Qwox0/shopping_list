use display_macro::display;
use leptos::*;
use uuid::Uuid;

use crate::language::{init_dict, dict};

/// replace with sections?
#[allow(unused)]
#[derive(Debug, Clone)]
pub enum State {
    Needed,
    Completed,
}

impl State {
    pub fn is_done(&self) -> bool {
        match self {
            State::Completed => true,
            _ => false,
        }
    }

    pub fn as_attribute(&self) -> String {
        match self {
            State::Completed => "completed",
            State::Needed => "needed",
        }
        .to_string()
    }
}

impl From<bool> for State {
    fn from(value: bool) -> Self {
        if value {
            State::Completed
        } else {
            State::Needed
        }
    }
}

#[derive(Debug, Clone)]
#[display("{} x{}", name.get(), amount.get())]
pub struct Item {
    pub id: Uuid,
    pub name: RwSignal<String>,
    pub amount: RwSignal<usize>,
    pub state: RwSignal<State>,
}

impl Item {
    pub fn new(cx: Scope, name: impl Into<String>, amount: usize) -> Self {
        Item {
            id: Uuid::new_v4(),
            name: create_rw_signal(cx, name.into()),
            amount: create_rw_signal(cx, amount),
            state: create_rw_signal(cx, State::Needed),
        }
    }
}

#[component]
pub fn Item(cx: Scope, item: Item) -> impl IntoView {
    let lang = init_dict!(cx);

    let delete = move |e| {
        let do_remove = window()
            .confirm_with_message(&format!("Do you want to remove {:?}?", item.name.get()))
            .unwrap_or_else(|jsVal| {
                log!("remove error {:?}", jsVal);
                false
            });
        log!("{}", do_remove);
    };

    view! { cx,
        <li state={move || item.state.get().as_attribute()} >
            <input type="checkbox"
                checked={move || item.state.get().is_done()}
                on:change=move |e| item.state.set(event_target_checked(&e).into())
            />
            <div class="text">
                {move || { format!("{}", item) }}
            </div>
            <div class="buttons">
                <input type="image"
                    alt="Edit"
                    src="/assets/pen.webp"
                    on:click=move |e| {}
                    />
                <input type="image"
                    alt={dict!(lang, |d| &d.delete)}
                    src="/assets/bin.webp"
                    on:click=delete
                />
            </div>
        </li>

    }
}
