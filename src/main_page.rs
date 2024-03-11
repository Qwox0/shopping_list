use crate::list::ListView;
use leptos::*;

#[component]
pub fn MainPage() -> impl IntoView {
    view! {
        <h1 style="text-align: center; margin: 1rem;">"Shopping List"</h1>

        <section>
            <button>
                "Add"
            </button>
            <label for="barcode-input">"Barcode Number:"</label>
            <input type="text" id="barcode-input"/>
        </section>

        <section id="list-sec">
            <ListView/>
        </section>
    }
}
