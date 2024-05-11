mod count;
pub mod data;
pub mod openfoodsfacts;
pub mod server_functions;
pub mod variant_data;

use self::{
    count::{ItemCount, ItemCountDisabled},
    data::{Item, PendingItem},
    variant_data::{PendingVariant, VariantImpl},
};
use crate::{
    barcode_scanner::{Barcode, BarcodeScanner},
    default_resource::DefaultResource,
    image::Image,
    item::{
        data::NewItem,
        server_functions::{
            set_amount, set_completed, InsertFromClient, InsertFromClientAction,
            InsertVariantFromClient, RemoveItem,
        },
        variant_data::{NewVariant, Variant},
    },
    list::ListResource,
    option_signal::OptionSignal,
    popup::{Popup, PopupSignal},
    server_sync_signal::ServerSyncSignal,
    subsignal::{subsignal, subsignals},
    util::{force_use_context, on_render, on_render_elem, SignalUpdateSome, SignalWithMap},
};
use leptos::{html::Div, *};
use web_sys::{Event, HtmlElement, MouseEvent};

fn stop_prop(f: impl Fn()) -> impl Fn(MouseEvent) {
    return move |ev| {
        ev.stop_propagation();
        f()
    };
}

#[derive(Clone, Copy)]
pub struct VariantsContainer(NodeRef<Div>);

#[component]
pub fn ItemView(item: Item) -> impl IntoView {
    let Item { id, amount, completed, variants } = item;

    let completed = ServerSyncSignal::new(completed, move |next| set_completed(id, next));
    let amount = ServerSyncSignal::new(amount, move |next| set_amount(id, next));

    let variants = create_rw_signal(variants);
    let new_variants = NewVariantsSignal::new();

    let is_expanded = create_rw_signal(false);
    let toggle_expand = move |_| is_expanded.update(|b| *b = !*b);
    let is_expanded = move || is_expanded() || !new_variants.is_empty();

    let list = force_use_context::<ListResource>();
    let remove_item = create_server_action::<RemoveItem>();
    let remove = move |_| match window().confirm_with_message("Remove Item?") {
        Ok(ok) if ok => {
            remove_item.dispatch(RemoveItem { id });
            list.0.update_some(|l| l.local_remove_id(id)); // FIXME: this produces a warning
        },
        _ => (),
    };

    let variants_container = NodeRef::new();
    provide_context(VariantsContainer(variants_container));

    on_render_elem(variants_container, |div| div.scroll_to_with_x_and_y(0.0, 0.0));

    view! {
        <li
            class="item"
            expanded=is_expanded
            checked=completed
        >
            <input
                type="checkbox"
                class="checkbox cursor-pointer"
                prop:checked=completed
                on:input=move |e| completed.set(event_target_checked(&e))
            />
            <div
                class="variants-container cursor-pointer"
                title="Expand"
                on:click=toggle_expand
                ref_=variants_container
            >
                <For
                    each=variants
                    key=|v| v.id
                    children=|variant| view! { <VariantView variant /> }
                />
                <For
                    each=move || new_variants.0().into_iter().enumerate()
                    key=|(idx, _)| *idx
                    let:t
                >
                    <NewVariantView variant=t.1 item_id=id />
                </For>
                <AddVariantButtonView new_variants/>
            </div>
            <div class="rhs">
                <ItemCount amount />
                <img
                    src="img/trash-alt-svgrepo-com.svg"
                    alt="Remove Item"
                    title="Remove Item"
                    class="remove-item-button cursor-pointer"
                    on:click=remove
                />
            </div>
        </li>
    }
}

impl IntoView for Item {
    fn into_view(self) -> View {
        ItemView(ItemViewProps { item: self }).into_view()
    }
}

#[component]
pub fn VariantView(variant: Variant) -> impl IntoView {
    let Variant { id, name, shop, barcode, brands, img_url, thumb_url, packaging, quantity } =
        variant;

    let edit_variant = || window().alert_with_message("TODO: Edit Variant").unwrap();
    let delete_variant = || window().alert_with_message("TODO: Delete Variant").unwrap();

    view! {
        <div class="variant">
            <div class="image">
                <Image thumb_url full_url=img_url/>
            </div>
            <div class="infos">
                <span class="name">{ name }</span>
                <span class="brands">{ brands }</span>
                <span class="quantity">{ quantity }</span>
                <div class="buttons">
                    <img
                        src="img/pen-square-svgrepo-com.svg"
                        //src="img/pen-svgrepo-com.svg"
                        alt="Edit Variant"
                        title="Edit Variant"
                        class="edit-variant-button cursor-pointer"
                        on:click=stop_prop(edit_variant)
                    />
                    <img
                        src="img/trash-alt-svgrepo-com.svg"
                        alt="Delete Variant"
                        title="Delete Variant"
                        class="delete-variant-button cursor-pointer"
                        on:click=stop_prop(delete_variant)
                    />
                </div>
            </div>
        </div>
    }
}

impl IntoView for Variant {
    fn into_view(self) -> View {
        VariantView(VariantViewProps { variant: self }).into_view()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct VariantSignal {
    barcode: OptionSignal<RwSignal<Option<Barcode>>>,
    variant: DefaultResource<Option<Barcode>, NewVariant>,
}

impl VariantSignal {
    pub fn new() -> Self {
        let barcode = OptionSignal::new();
        let variant = DefaultResource::new_local(
            barcode,
            move |barcode| async move {
                match barcode {
                    Some(barcode) => NewVariant::from_barcode(barcode)
                        .await
                        /*
                        .inspect(|a| window().alert_with_message(&format!("{:?}", a)).unwrap())
                        .inspect_err(|e| {
                            window()
                                .alert_with_message(&format!("ItemData::from_barcode error: {}", e))
                                .unwrap()
                        })
                        */
                        .inspect_err(|e| logging::error!("ItemData::from_barcode error: {}", e))
                        .ok(),
                    None => None,
                }
                .unwrap_or_else(NewVariant::default)
            },
            NewVariant::default,
        );
        VariantSignal { barcode, variant }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct NewVariantsSignal(pub RwSignal<Vec<VariantSignal>>);

impl NewVariantsSignal {
    pub fn new() -> Self {
        Self(RwSignal::new(vec![]))
    }

    pub fn add_empty_variant(&self) {
        self.0.update(|v| v.push(VariantSignal::new()));
    }

    pub fn reset_to_empty(&self) {
        self.0.update(|v| v.clear())
    }

    pub fn reset_to_one_variant(&self) {
        self.0.update(|v| match v.len() {
            0 => v.push(VariantSignal::new()),
            1 => {
                v[0].barcode.reset();
                v[0].variant.reset();
            },
            _ => drop(v.drain(1..)),
        })
    }

    pub fn to_variants_vec(&self) -> Vec<NewVariant> {
        self.0.with(|v| v.iter().map(|t| t.variant.get()).collect())
    }

    pub fn is_empty(&self) -> bool {
        self.0.with(Vec::is_empty)
    }
}

#[component]
pub fn NewItemView(show: RwSignal<bool>) -> impl IntoView {
    let default_item = NewItem::default();
    let amount = create_rw_signal(default_item.amount);
    let completed = create_rw_signal(default_item.completed);
    let new_variants = NewVariantsSignal::new();
    new_variants.add_empty_variant();

    let reset = move || {
        let default_item = NewItem::default();
        amount.set(default_item.amount);
        completed.set(default_item.completed);
        new_variants.reset_to_one_variant();
    };

    // let variants_vec = subsignal(item, |i| &i.variants, |i| &mut i.variants);
    // let variants = move || subsignals(variants_vec);

    let item = move || NewItem {
        id: (),
        amount: amount(),
        completed: completed(),
        variants: new_variants.to_variants_vec(),
    };

    create_effect(move |_| {
        logging::log!("new_variants: {:?}", new_variants.0());
    });

    let insert_from_client = force_use_context::<InsertFromClientAction>();
    let add_item = move |_| {
        if new_variants
            .0
            .with(|v| v.iter().any(|i| i.variant.get().name.trim().is_empty()))
        {
            window().alert_with_message("empty Name not allowed").unwrap();
        } else {
            insert_from_client.0.dispatch(InsertFromClient { new_item: item() });
        };
        reset();
        show.set(false);
    };

    let variants_container = NodeRef::new();
    provide_context(VariantsContainer(variants_container));

    view! {
        <li class="item new" expanded hidden=move || !show()>
            <input
                type="checkbox"
                class="checkbox cursor-pointer"
                prop:checked=move || completed()
                on:input=move |e| completed.set(event_target_checked(&e))
            />
            <div
                class="variants-container"
                ref_=variants_container
            >
                <For
                    each=move || new_variants.0().into_iter().enumerate()
                    key=|(idx, _)| *idx
                    let:t
                >
                    <NewVariantView variant=t.1 />
                </For>
                <AddVariantButtonView new_variants />
            </div>
            <div class="rhs">
                <ItemCount amount />
                <img
                    src="img/check-svgrepo-com.svg"
                    alt="Save Item"
                    title="Save Item"
                    class="save-item-button cursor-pointer"
                    on:click=add_item
                />
            </div>
        </li>
    }
}

#[component]
pub fn NewVariantView(
    variant: VariantSignal,
    /// [`ItemView`] -> `Some(id)`
    /// [`NewItemView`] -> `None`
    #[prop(optional)]
    item_id: Option<i64>,
) -> impl IntoView {
    let container = NodeRef::<Div>::new();
    on_render_elem(container, move |container| {
        let variant_pos = HtmlElement::offset_left(&container);
        force_use_context::<VariantsContainer>()
            .0
            .get()
            .expect("variants container has been loaded")
            .scroll_to_with_x_and_y(variant_pos as f64, 0.0)
    });

    let barcode_popup = PopupSignal::new();

    let VariantSignal { barcode, variant } = variant;
    let name = subsignal!(variant => name);
    let shop = subsignal!(variant => shop);
    let brands = subsignal!(variant => brands);
    let img_url = subsignal!(variant => img_url);
    let thumb_url = subsignal!(variant => thumb_url);
    let thumbnail = move || {
        thumb_url
            .with_map(|u| format!("url(\"{u}\")"))
            .unwrap_or_else(|| "unset".to_string())
    };
    let packaging = subsignal!(variant => packaging);
    let quantity = subsignal!(variant => quantity);

    let add_variant = move |_| match item_id {
        Some(_) if name.with(|s| s.trim().is_empty()) => {
            window().alert_with_message("empty Name not allowed").unwrap()
        },
        Some(item_id) => create_server_action()
            .dispatch(InsertVariantFromClient { item_id, new_variant: variant() }),
        None => (),
    };
    let discard_variant = |_| window().alert_with_message("TODO: Discard Variant").unwrap();

    view! {
        <div
            class="variant new"
            ref_=container
        >
            <div
                class="barcode-scanner image cursor-pointer"
                on:click=move |_| barcode_popup.open()
                style:background-image=thumbnail
            >
                <img
                    src="img/barcode-outline.svg"
                    alt="Scan Barcode"
                    title="Scan Barcode"
                />
                <Popup popup=barcode_popup>
                    <BarcodeScanner set_barcode=move |b| {
                        barcode.set(b);
                        barcode_popup.close();
                    } />
                </Popup>
            </div>
            //<input type="file" accept="image/*" class="image-input" />
            <div class="infos">
                <input type="text"
                    class="name"
                    placeholder="Name"
                    title="Name"
                    prop:value=name
                    on:change=move |ev| name.set(event_target_value(&ev))
                />
                <input type="text"
                    class="brands"
                    placeholder="Brands"
                    title="Brands"
                    prop:value=move || brands()
                    on:change=move |ev| brands.set(event_target_value(&ev))
                />
                <input type="text"
                    class="quantity"
                    placeholder="Quantity"
                    title="Quantity"
                    prop:value=move || quantity()
                    on:change=move |ev| quantity.set(event_target_value(&ev))
                />
                <div class="buttons">
                    <Show when=move || item_id.is_some()>
                        <img
                            src="img/check-svgrepo-com.svg"
                            alt="Add Variant"
                            title="Add Variant"
                            class="add-variant-button cursor-pointer"
                            on:click=add_variant
                        />
                    </Show>
                    <img
                        src="img/trash-alt-svgrepo-com.svg"
                        alt="Discard Variant"
                        title="Discard Variant"
                        class="discard-variant-button cursor-pointer"
                        on:click=discard_variant
                    />
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn AddVariantButtonView(new_variants: NewVariantsSignal) -> impl IntoView {
    let add_new_variant = move || new_variants.add_empty_variant();
    view! {
        <div
            class="add-variant cursor-pointer"
            title="Add new Item Variant"
            on:click=stop_prop(add_new_variant)
        >
            <img
                src="img/plus-svgrepo-com.svg"
                alt="Add new Item Variant"
                class="new-variant-button"
            />
        </div>
    }
}

#[component]
pub fn PendingItemView(item: PendingItem) -> impl IntoView {
    let NewItem { id, amount, completed, variants } = item.0;

    let variants = create_rw_signal(variants);

    let is_expanded = create_rw_signal(false);
    let toggle_expand = move |_| is_expanded.update(|b| *b = !*b);
    let is_expanded = move || is_expanded();

    view! {
        <li
            class="item pending"
            expanded=is_expanded
            checked=completed
        >
            <input
                type="checkbox"
                class="checkbox"
                prop:checked=completed
                disabled
            />
            <div
                class="variants-container cursor-pointer"
                title="Expand"
                on:click=toggle_expand
            >
                <For
                    each=variants
                    key=|v| v.id
                    children=|variant| view! { <PendingVariantView variant /> }
                />
            </div>
            <div class="rhs">
                <ItemCountDisabled amount />
            </div>
        </li>
    }
}

impl IntoView for PendingItem {
    fn into_view(self) -> View {
        PendingItemView(PendingItemViewProps { item: self }).into_view()
    }
}

#[component]
pub fn PendingVariantView(#[prop(into)] variant: PendingVariant) -> impl IntoView {
    let VariantImpl { name, shop, brands, img_url, thumb_url, packaging, quantity, .. } = variant.0;

    view! {
        <div class="variant pending">
            <div class="image">
                <Image thumb_url full_url=img_url/>
            </div>
            <div class="infos">
                <span class="name">{ name }</span>
                <span class="brands">{ brands }</span>
                <span class="quantity">{ quantity }</span>
                /*
                <div class="buttons">
                    <img
                        src="img/trash-alt-svgrepo-com.svg"
                        alt="Delete Variant"
                        class="delete-variant-button cursor-pointer"
                        on:click=stop_prop(delete_variant)
                    />
                </div>
                */
            </div>
        </div>
    }
}

impl IntoView for PendingVariant {
    fn into_view(self) -> View {
        PendingVariantView(PendingVariantViewProps { variant: self }).into_view()
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ShowNewItem(pub RwSignal<bool>);

impl ShowNewItem {
    pub fn toggle(&self) {
        self.0.update(|b| *b = !*b);
    }
}
