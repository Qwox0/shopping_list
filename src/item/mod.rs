mod count;
pub mod data;
pub mod openfoodsfacts;
pub mod server_functions;
pub mod variant_data;

use self::{count::ItemCount, data::Item};
use crate::{
    barcode_scanner::BarcodeScanner,
    default_resource::DefaultResource,
    image::Image,
    item::{
        data::NewItem,
        server_functions::{set_amount, set_completed},
        variant_data::{ItemVariant, NewItemVariant},
    },
    option_signal::OptionSignal,
    popup::{Popup, PopupSignal},
    server_sync_signal::ServerSyncSignal,
    subsignal::{subsignal, subsignals},
};
use leptos::*;

#[component]
pub fn ItemView(item: Item) -> impl IntoView {
    let Item { id, amount, completed, variants } = item;

    let completed = ServerSyncSignal::new(completed, move |next| set_completed(id, next));
    let amount = ServerSyncSignal::new(amount, move |next| set_amount(id, next));

    let variants = create_rw_signal(variants);

    let is_expanded = create_rw_signal(false);

    view! {
        <li
            class="item"
            expanded=is_expanded
            checked=completed
        >
            <input
                type="checkbox"
                class="checkbox"
                prop:checked=completed
                on:input=move |e| completed.set(event_target_checked(&e))
            />
            <div class="variants-container">
                <For
                    each=variants
                    key=|v| v.id
                    let:item_variant
                >
                    <ItemVariantView item_variant is_expanded />
                </For>
                <AddNewItemVariantView />
            </div>
            <ItemCount amount />
        </li>
    }
}

#[component]
pub fn ItemVariantView(item_variant: ItemVariant, is_expanded: RwSignal<bool>) -> impl IntoView {
    let ItemVariant { id, name, shop_id, barcode, brands, img_url, thumb_url, packaging, quantity } =
        item_variant;

    let toggle_expand = move |_| is_expanded.update(|b| *b = !*b);

    view! {
        <div class="variant">
            <Image thumb_url full_url=img_url/>
            <div
                class="infos"
                title="Expand"
                class="name cursor-pointer"
                on:click=toggle_expand
            >
                <span class="name">{ name }</span>
                <span class="brands">{ brands }</span>
                <span class="quantity">{ quantity }</span>
            </div>
        </div>
    }
}

#[component]
pub fn NewItemView<H>(hidden: H) -> impl IntoView
where H: Fn() -> bool + 'static {
    let barcode_popup = PopupSignal::new();

    let new_item_barcode = OptionSignal::new();
    let item = DefaultResource::new_local(
        new_item_barcode,
        move |barcode| async move {
            match barcode {
                Some(barcode) => NewItem::from_barcode(barcode)
                    .await
                    .inspect_err(|e| logging::error!("ItemData::from_barcode error: {}", e))
                    .ok(),
                None => None,
            }
            .unwrap_or_else(NewItem::with_default_variant)
        },
        NewItem::with_default_variant,
    );

    create_effect(move |_| {
        logging::log!("item: {:?}", item());
    });

    let amount = subsignal(item, |i| &i.amount, |i| &mut i.amount);
    let completed = subsignal(item, |i| &i.completed, |i| &mut i.completed);
    let variants = subsignal(item, |i| &i.variants, |i| &mut i.variants);
    let variants = move || subsignals(variants);

    create_effect(move |_| {
        logging::log!("barcode: {:?}", new_item_barcode());
    });

    view! {
        <li class="new-item" expanded hidden=hidden>
            <input
                type="checkbox"
                class="checkbox"
                prop:checked=move || completed()
                on:input=move |e| completed.set(event_target_checked(&e))
            />
            <div class="variants-container">
                <For
                    each=variants
                    key=|v| v.with(|i| i.id)
                    let:item_variant
                >
                    <NewItemVariantView item_variant />
                </For>
                <AddNewItemVariantView />
            </div>
            <ItemCount amount />
            /*
            <div class="buttons">
                <img
                    src="img/barcode-scan-svgrepo-com.svg"
                    class="cursor-pointer"
                    on:click=move |_| barcode_popup.open()
                />
                <img
                    src="img/check-svgrepo-com.svg"
                    class="cursor-pointer"
                    on:click=move |_| { window().alert_with_message("save"); }
                />
            </div>
            <Popup popup=barcode_popup>
                <BarcodeScanner set_barcode=move |res| {
                    set_new_item_barcode(res.unwrap());
                    barcode_popup.close();
                } />
            </Popup>
            */
        </li>
    }
}

#[component]
/*
pub fn NewItemVariantView<Sig, F, G>(item_variant: SubRWSignal<Sig, F, G>) -> impl IntoView
where
    Sig: SignalWith<Value = NewItemVariant>,
    F: Fn(&Sig::Value) -> &NewItemVariant,
{
*/
//pub fn NewItemVariantView(#[prop(into)] item_variant: Signal<NewItemVariant>)
// -> impl IntoView {
//pub fn NewItemVariantView(item_variant: NewItemVariant) -> impl IntoView {
pub fn NewItemVariantView<Sig>(item_variant: Sig) -> impl IntoView
where Sig: SignalWith<Value = NewItemVariant> + SignalGet<Value = NewItemVariant> {
    let new_item_barcode = OptionSignal::new();
    let item = DefaultResource::new_local(
        new_item_barcode,
        move |barcode| async move {
            match barcode {
                Some(barcode) => NewItemVariant::from_barcode(barcode)
                    .await
                    .inspect_err(|e| logging::error!("NewItemVariant::from_barcode error: {}", e))
                    .ok(),
                None => None,
            }
            .unwrap_or_else(NewItemVariant::default)
        },
        NewItemVariant::default,
    );

    let NewItemVariant {
        id,
        name,
        shop_id, // TODO
        barcode,
        brands,
        img_url,
        thumb_url,
        packaging,
        quantity,
    } = item_variant.get();

    let barcode_popup = PopupSignal::new();

    view! {
        <div class="variant">
            //<input type="file" accept="image/*" class="image-input" />
            <img
                src="img/barcode-scan-svgrepo-com.svg"
                class="barcode-scanner cursor-pointer"
                on:click=move |_| barcode_popup.open()
            />
            <Popup popup=barcode_popup>
                <BarcodeScanner set_barcode=move |res| {
                    new_item_barcode.set(res.unwrap());
                    barcode_popup.close();
                } />
            </Popup>
            <div
                class="infos"
                title="Expand"
                class="name cursor-pointer"
                //on:click=toggle_expand
            >
                <span class="name">{ &name }</span>
                <input type="text" class="name"
                    placeholder="Name"
                    prop:value=name
                    on:change=move |ev| {
                        let text = event_target_value(&ev);
                        //item.update(|i| i.as_mut().do_(|i| i.variants[0].name = text))
                    }
                />
                <span class="brands">{ brands }</span>
                <span class="quantity">{ quantity }</span>
            </div>
        </div>
    }
}

#[component]
pub fn AddNewItemVariantView() -> impl IntoView {
    view! {
        <div class="add-variant">
            <img
                src="img/plus-svgrepo-com.svg"
                title="Add new Item Variant"
                class="new-variant-button cursor-pointer"
            />
        </div>
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ShowNewItem(pub RwSignal<bool>);

impl ShowNewItem {
    pub fn toggle(&self) {
        self.0.update(|b| *b = !*b);
    }
}
