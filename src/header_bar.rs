use crate::{
    item::{RefreshList, ShowNewItem},
    util::force_use_context,
};
use leptos::*;
use web_sys::{ScrollBehavior, ScrollToOptions};

#[component]
pub fn HeaderBar() -> impl IntoView {
    let show_new_item = force_use_context::<ShowNewItem>();
    let refresh_list = force_use_context::<RefreshList>().0;
    let new_item_active = move || show_new_item.0.get();

    let toggle_new_item = move |_| {
        if !show_new_item.0.get_untracked() {
            window().scroll_to_with_scroll_to_options(
                ScrollToOptions::new().top(0.0).behavior(ScrollBehavior::Smooth),
            );
        }
        show_new_item.toggle();
    };

    let tooltip = move || if show_new_item.0.get() { "Discard new Item" } else { "Add new Item" };

    view! {
        <header id="header-bar">
            <div class="header-bar--left">
                <img
                    src="img/refresh-cw-svgrepo-com.svg"
                    alt="Refresh"
                    title="Refresh"
                    class="refresh-button cursor-pointer"
                    on:click=move |_| refresh_list.notify()
                />
            </div>
            <div class="header-bar--center">
                <h2>"Shopping List"</h2>
            </div>
            <div class="header-bar--right">
                /*
                <img
                    src="img/check-svgrepo-com.svg"
                    alt="Save Item"
                    title="Save Item"
                    class="save-item-button cursor-pointer"
                    class:new-item-active=new_item_active
                    on:click=move |_| { let _ = window().alert_with_message("save"); }
                />
                */
                <img
                    src="img/plus-large-svgrepo-com.svg"
                    alt=tooltip
                    title=tooltip
                    class="new-item-button cursor-pointer"
                    class:new-item-active=new_item_active
                    on:click=toggle_new_item
                />
            </div>
        </header>
    }
}
