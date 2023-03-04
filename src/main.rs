const SOCKET_ADDRESS: &str = "0.0.0.0:33080";

#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //std::env::set_var("RUST_BACKTRACE", "full");
    std::env::set_var("RUST_BACKTRACE", "1");
    use actix_files::Files;
    use actix_web::*;
    use leptos::*;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};
    use shopping_list::view::app::app;

    fn get_ssl_builder() -> anyhow::Result<SslAcceptorBuilder> {
        let mut ssl_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;
        ssl_builder.set_private_key_file("privkey.pem", SslFiletype::PEM)?;
        ssl_builder.set_certificate_chain_file("fullchain.pem")?;
        Ok(ssl_builder)
    }

    shopping_list::state::register_server_functions();

    let conf = get_configuration(Some("./Cargo.toml")).await.unwrap();
    let addr = conf.leptos_options.site_addr;
    // Generate the list of routes in your Leptos App

    let server = HttpServer::new(move || {
        let leptos_options = &conf.leptos_options;
        let routes = generate_route_list(app);
        App::new()
            .wrap(middleware::Compress::default())
            .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
            .leptos_routes(leptos_options.clone(), routes, app)
            .service(Files::new("/", &leptos_options.site_root))
    })
    .bind(&addr)?;
    log!("bind {} with ssl", &addr);
    if let Some(ssl_builder) = get_ssl_builder().ok() {
        log!("bind {}", SOCKET_ADDRESS);
        server.bind_openssl(SOCKET_ADDRESS, ssl_builder)?
    } else {
        server
    }
    .run()
    .await
}
