use crate::{language::text_macro::text, state::item::Item};
use leptos::*;

#[component]
pub fn Item(cx: Scope, item: Item) -> impl IntoView {
    let delete = move |_e| {
        let do_remove = window()
            .confirm_with_message(&format!("Do you want to remove {:?}?", item.name.get()))
            .unwrap_or_else(|jsVal| {
                log!("remove error {:?}", jsVal);
                false
            });
        log!("{}", do_remove);
    };

    view! { cx,
        <li state=move || item.state.get().as_attribute() >
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
        </li>

    }
}
