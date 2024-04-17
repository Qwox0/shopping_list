use leptos::*;

#[component]
pub fn ItemCount() -> impl IntoView {
    let (count, set_count) = create_signal(0u32);
    let inc = move |_| set_count.update(|x| *x = x.saturating_add(1));
    let dec = move |_| set_count.update(|x| *x = x.saturating_sub(1));
    view! {
        <div class="item-count">
            <button on:click=inc>"+"</button>
            <input
                type="number"
                min="0"
                prop:value=count
                on:change=move |ev| set_count.update(
                    |c| *c = event_target_value(&ev).parse().unwrap_or(*c)
                )
            />
            <button on:click=dec>"-"</button>
        </div>
    }
}
