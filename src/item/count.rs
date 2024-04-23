use leptos::*;

#[component]
pub fn ItemCount<S>(amount: S) -> impl IntoView
where S: SignalUpdate<Value = u64> + SignalGet<Value = u64> + Copy + 'static {
    let inc = move |_| amount.update(|x| *x = x.saturating_add(1));
    let dec = move |_| amount.update(|x| *x = x.saturating_sub(1));
    view! {
        <div class="item-count">
            <button on:click=inc>"+"</button>
            <input
                type="number"
                min="0"
                value=move || amount.get()
                prop:value=move || amount.get()
                on:change=move |ev| amount.update(
                    |c| *c = event_target_value(&ev).parse().unwrap_or(*c)
                )
            />
            <button on:click=dec>"-"</button>
        </div>
    }
}
