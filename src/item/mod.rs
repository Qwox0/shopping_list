mod count;
pub mod data;
pub mod openfoodsfacts;
pub mod variant_data;

use self::{count::ItemCount, data::Item};
use crate::{
    barcode_scanner::BarcodeScanner,
    image::Image,
    item::{
        data::NewItem,
        variant_data::{ItemVariant, NewItemVariant},
    },
    option_signal::create_option_signal,
    popup::{Popup, PopupSignal},
    subsignal::subsignal,
};
use leptos::*;

#[component]
pub fn ItemView(item: Item) -> impl IntoView {
    let Item { id, amount, completed, variants } = item;
    let variants = create_signal(variants).0;
    view! {
        <li class="item">
            <input type="checkbox" checked=completed/>
            //<Image thumb_url full_url=img_url/>
            //<span>{ name }</span>
            <div class="variants-container">
                <For
                    each=variants
                    key=|v| v.id
                    children=|item_variant| view! { <ItemVariantView item_variant /> }
                />
            </div>
            <ItemCount />
        </li>
    }
}

#[component]
/*
pub fn ItemVariantView<Sig, F, G>(item_variant: SubRWSignal<Sig, F, G>) -> impl IntoView
where
    Sig: SignalWith<Value = ItemVariant>,
    F: Fn(&Sig::Value) -> &ItemVariant,
{
*/
//pub fn ItemVariantView(#[prop(into)] item_variant: Signal<ItemVariant>) ->
// impl IntoView {
pub fn ItemVariantView(item_variant: ItemVariant) -> impl IntoView {
    //
    let ItemVariant { name, img_url, thumb_url, .. } = item_variant;
    view! {
        <div class="variant">
            <Image thumb_url full_url=img_url/>
            <span>{ name }</span>
        </div>
    }
}

#[component]
pub fn NewItemView<H>(hidden: H) -> impl IntoView
where H: Fn() -> bool + 'static {
    let barcode_popup = PopupSignal::new();
    let (new_item_barcode, set_new_item_barcode) = create_option_signal();

    let item = create_local_resource_with_initial_value(
        new_item_barcode,
        move |barcode| async move {
            if let Some(barcode) = barcode {
                let a = NewItem::from_barcode(barcode).await;
                if let Err(e) = a.as_ref() {
                    logging::error!("ItemData::from_barcode error: {}", e);
                }
                a.unwrap_or_default()
            } else {
                NewItem { variants: vec![NewItemVariant::default()], ..NewItem::default() }
            }
        },
        Some(NewItem::default()),
    );
    let variants =
        subsignal(item, |i| &i.as_ref().unwrap().variants, |i| &mut i.as_mut().unwrap().variants);
    //let variants = subsignals(variants);

    create_effect(move |_| {
        logging::log!("barcode: {:?}", new_item_barcode());
    });

    view! {
        <div class="new-item" hidden=hidden>
            <input type="checkbox"/>
            <div class="variants-container">
                <For
                    each=variants
                    key=|v| v.id //v.with(|i| i.id)
                    children=|item_variant| view! { <NewItemVariantView item_variant /> }
                />
                <AddNewItemVariantView />
            </div>
            <ItemCount />
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
        </div>
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
pub fn NewItemVariantView(item_variant: NewItemVariant) -> impl IntoView {
    let NewItemVariant { name, img_url, thumb_url, .. } = item_variant;

    let barcode_popup = PopupSignal::new();
    let (new_item_barcode, set_new_item_barcode) = create_option_signal();

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
                    set_new_item_barcode(res.unwrap());
                    barcode_popup.close();
                } />
            </Popup>
            <input type="text" class="name-input"
                placeholder="Name"
                prop:value=name
                on:change=move |ev| {
                    let text = event_target_value(&ev);
                    //item.update(|i| i.as_mut().do_(|i| i.variants[0].name = text))
                }
            />
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
