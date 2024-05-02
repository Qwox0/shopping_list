use crate::{
    barcode_scanner::Barcode,
    item::{data::NewItem, openfoodsfacts, server_functions::add_item_from_barcode},
};
use leptos::*;
use leptos_router::*;

#[server]
pub async fn db_action(barcode: String, action: String) -> Result<String, ServerFnError> {
    let barcode = Barcode::try_from(barcode)?;

    match action.as_str() {
        "request json" => Ok(format!("{:#}", openfoodsfacts::request_with_barcode(barcode).await?)),
        "request OpenFoodFactsProduct" => Ok(format!(
            "{:#?}",
            openfoodsfacts::OpenFoodFactsProduct::request_with_barcode(barcode).await?
        )),
        "request ItemData" => Ok(format!("{:#?}", NewItem::from_barcode(barcode).await?)),
        "Add Item" => add_item_from_barcode(barcode).await.map(|_| format!("Added Item")),
        _ => Err(ServerFnError::new(format!("invalid action: {:?}", action))),
    }
}

#[component]
pub fn DBTool() -> impl IntoView {
    let action = create_server_action::<DbAction>();
    let output = action.value();
    let text = move || match output().transpose() {
        Ok(s) => s.unwrap_or_default(),
        Err(err) => format!("ERROR: {err}"),
    };

    view! {
        <h1>"DB Tool"</h1>
        <ActionForm action=action>
            <label for="barcode-input">"barcode: "</label>
            <input type="text" id="barcode-input" name="barcode"/>
            <br/>
            <input type="submit" name="action" value="request json"/>
            <input type="submit" name="action" value="request OpenFoodFactsProduct"/>
            <input type="submit" name="action" value="request ItemData"/>
            <input type="submit" name="action" value="Add Item"/>
        </ActionForm>
        <br/>
        <textarea
            prop:value=text
            onmouseover="this.style.height = this.scrollHeight + 10 + \"px\""
            style:width="100%"
        />
    }
}
