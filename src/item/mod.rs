mod count;
pub mod data;
pub mod openfoodsfacts;
pub mod server_functions;
pub mod variant_data;

use self::{count::ItemCount, data::Item};
use crate::{
    barcode_scanner::{Barcode, BarcodeScanner},
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
    let new_variants = create_rw_signal(Vec::<ItemVariantSignal>::new());

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
                <For
                    each=new_variants
                    key=|t| t.item_variant.with(|i| i.id)
                    let:item_variant
                >
                    <NewItemVariantView item_variant />
                </For>
                <AddNewItemVariantView new_variants/>
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
            <div class="image">
                <Image thumb_url full_url=img_url/>
            </div>
            <div
                title="Expand"
                class="infos cursor-pointer"
                on:click=toggle_expand
            >
                <span class="name">{ name }</span>
                <span class="brands">{ brands }</span>
                <span class="quantity">{ quantity }</span>
            </div>
        </div>
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ItemVariantSignal {
    barcode: OptionSignal<RwSignal<Option<Barcode>>>,
    item_variant: DefaultResource<Option<Barcode>, NewItemVariant>,
}

impl ItemVariantSignal {
    pub fn new() -> Self {
        let barcode = OptionSignal::new();
        let item_variant = DefaultResource::new_local(
            barcode,
            move |barcode| async move {
                match barcode {
                    Some(barcode) => NewItemVariant::from_barcode(barcode)
                        .await
                        .inspect_err(|e| logging::error!("ItemData::from_barcode error: {}", e))
                        .ok(),
                    None => None,
                }
                .unwrap_or_else(NewItemVariant::default)
            },
            NewItemVariant::default,
        );
        ItemVariantSignal { barcode, item_variant }
    }
}

#[component]
pub fn NewItemView<H>(hidden: H) -> impl IntoView
where H: Fn() -> bool + 'static {
    let default_item = NewItem::default();
    let amount = create_rw_signal(default_item.amount);
    let completed = create_rw_signal(default_item.completed);
    let new_variants = create_rw_signal(vec![ItemVariantSignal::new()]);

    // let variants_vec = subsignal(item, |i| &i.variants, |i| &mut i.variants);
    // let variants = move || subsignals(variants_vec);

    let item = move || NewItem {
        id: (),
        amount: amount(),
        completed: completed(),
        variants: new_variants.with(|v| v.iter().map(|t| t.item_variant.get()).collect()),
    };

    create_effect(move |_| {
        logging::log!("item: {:?}", item());
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
                    each=new_variants
                    key=|t| t.item_variant.with(|i| i.id)
                    let:item_variant
                >
                    <NewItemVariantView item_variant />
                </For>
                <AddNewItemVariantView new_variants />
            </div>
            <ItemCount amount />
        </li>
    }
}

#[component]
pub fn NewItemVariantView(item_variant: ItemVariantSignal) -> impl IntoView {
    let barcode_popup = PopupSignal::new();

    let ItemVariantSignal { barcode, item_variant } = item_variant;
    let name = subsignal!(item_variant => name);
    let shop_id = subsignal!(item_variant => shop_id);
    let brands = subsignal!(item_variant => brands);
    let img_url = subsignal!(item_variant => img_url);
    let thumb_url = subsignal!(item_variant => thumb_url);
    let packaging = subsignal!(item_variant => packaging);
    let quantity = subsignal!(item_variant => quantity);

    create_effect(move |_| {
        println!("{:?}", barcode_popup.is_open());
    });

    view! {
        <div class="new-variant">
            <div
                class="barcode-scanner image cursor-pointer"
                on:click=move |_| {
                    logging::log!("click");
                    barcode_popup.open()}
                style:background-image=thumb_url
            >
                <img src="img/barcode-scan-svgrepo-com.svg" />
                <Popup popup=barcode_popup>
                    <BarcodeScanner set_barcode=move |b| {
                        barcode.set(b);
                        barcode_popup.close();
                    } />
                </Popup>
            </div>
            //<input type="file" accept="image/*" class="image-input" />
            <div
                title="Expand"
                class="infos cursor-pointer"
                //on:click=toggle_expand
            >
                <input type="text"
                    class="name"
                    placeholder="Name"
                    prop:value=name
                    on:change=move |ev| name.set(event_target_value(&ev))
                />
                <input type="text"
                    class="brands"
                    placeholder="Brands"
                    prop:value=move || brands().unwrap_or_default()
                    on:change=move |ev| brands.set(Some(event_target_value(&ev)))
                />
                <input type="text"
                    class="quantity"
                    placeholder="Quantity"
                    prop:value=move || quantity().unwrap_or_default()
                    on:change=move |ev| quantity.set(Some(event_target_value(&ev)))
                />
            </div>
        </div>
    }
}

#[component]
pub fn AddNewItemVariantView<Sig>(new_variants: Sig) -> impl IntoView
where Sig: SignalUpdate<Value = Vec<ItemVariantSignal>> + 'static {
    let add_new_variant = move |_| new_variants.update(|v| v.push(ItemVariantSignal::new()));

    view! {
        <div
            class="add-variant"
            on:click=add_new_variant
        >
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
