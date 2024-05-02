use leptos::*;
use leptos_router::ActionForm;

#[server]
pub async fn login(list_name: String) -> Result<(), ServerFnError> {
    leptos_axum::redirect("/");
    Ok(())
}

#[component]
pub fn LoginView() -> impl IntoView {
    let login = create_server_action::<Login>();

    view! {
        <div id="login">
            <div class="login-box">
                <ActionForm action=login>
                    <input type="text"
                        name="list_name"
                        title="List name"
                        placeholder="List name"
                        class="list-name"
                    />
                </ActionForm>
            </div>
        </div>
    }
}
