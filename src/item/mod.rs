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
    util::OptionDo,
};
use leptos::*;

#[component]
pub fn ItemView(item: Item) -> impl IntoView {
    let ItemVariant { name, img_url, thumb_url, .. } = item.variants[0].clone(); // TODO
    view! {
        <li class="item">
            <input type="checkbox"/>
            <Image thumb_url full_url=img_url/>
            <span>{ name }</span>
            <ItemCount />
        </li>
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

    create_effect(move |_| {
        logging::log!("barcode: {:?}", new_item_barcode());
    });

    view! {
        <div class="new-item" hidden=hidden>
            <input type="checkbox"/>
            <Image thumb_url=Some("".to_string()) full_url=Some("".to_string())/>
            <input type="text" id="name-input"
                placeholder="Name"
                prop:value=move || item.with(|i| i.as_ref().map(|i| i.variants[0].name.clone()).unwrap_or_default())
                on:change=move |ev| {
                    let text = event_target_value(&ev);
                    item.update(|i| i.as_mut().do_(|i| i.variants[0].name = text))
                }
            />
            <ItemCount />
            <div class="new-item--buttons">
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
