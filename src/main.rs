const SOCKET_ADDRESS: &str = "0.0.0.0:33080";

#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    fn app(cx: leptos::Scope) -> impl IntoView {
        use shopping_list::view::app::*;
        view! { cx, <App />}
    }
    //std::env::set_var("RUST_BACKTRACE", "1");
    use actix_files::Files;
    use actix_web::*;
    use leptos::*;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
    use shopping_list::view::app::*;

    register_server_functions();

    let mut ssl_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;
    ssl_builder.set_private_key_file("privkey16.pem", SslFiletype::PEM)?;
    ssl_builder.set_certificate_chain_file("fullchain16.pem")?;

    let conf = get_configuration(Some("./Cargo.toml")).await.unwrap();
    let addr = conf.leptos_options.site_address;
    // Generate the list of routes in your Leptos App

    HttpServer::new(move || {
        let leptos_options = &conf.leptos_options;
        let routes = generate_route_list(app);
        App::new()
            .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
            .leptos_routes(leptos_options.clone(), routes, app)
            .service(Files::new("/", &leptos_options.site_root))
            .wrap(middleware::Compress::default())
    })
    .bind(&addr)?
    .bind_openssl(SOCKET_ADDRESS, ssl_builder)?
    .run()
    .await
}
