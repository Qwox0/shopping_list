use crate::{
    state::{app_state::AppState, item::Item},
    view::text::text,
};
use leptos::*;

#[component]
pub fn ItemView(cx: Scope, item: Item) -> impl IntoView {
    let list = AppState::from_context(cx).item_list;

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
