const PORT: u16 = 33080;

#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //std::env::set_var("RUST_BACKTRACE", "full");
    std::env::set_var("RUST_BACKTRACE", "1");
    use actix_files::Files;
    use actix_web::{dev::Service, *};
    use futures_util::future::{self, Either};
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
            .wrap_fn(move |sreq, srv| {
                let host = sreq.connection_info().host().to_owned();
                let uri = sreq.uri().to_owned();
                let url = format!("https://{host}{uri}");

                // If the scheme is "https" then it will let other services below this wrap_fn
                // handle the request and if it's "http" then a response with redirect status code
                // will be sent whose "location" header will be same as before, with just "http"
                // changed to "https"
                /*
                log!("host: {host:?}");
                log!("uri: {uri:?}");
                log!("uri.to_string(): {:?}", uri.to_string());
                log!("url: {url:?}");
                log!("url.to_string(): {:?}", url.to_string());
                log!("port: {:?}", addr.port());
                log!("contains: {:?}", uri.to_string().contains(&format!(":{}", addr.port())));
                log!("contains2: {:?}", url.to_string().contains(&format!(":{}", addr.port())));
                */

                if sreq.connection_info().scheme() == "https" || url.to_string().contains(&format!(":{}", addr.port())) {
                    Either::Left(srv.call(sreq)) //.map(|res| res))
                } else {
                    println!("An http request has arrived here, i will redirect it to use https");
                    return Either::Right(future::ready(Ok(sreq.into_response(
                        HttpResponse::MovedPermanently()
                            .append_header((http::header::LOCATION, url))
                            .finish(),
                    ))));
                }
            })
            .wrap(middleware::Compress::default())
            .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
            .leptos_routes(leptos_options.clone(), routes, app)
            .service(Files::new("/", &leptos_options.site_root))
    })
    .bind(&addr)?; // for test only
    log!("bind: {}", addr);
    if let Some(ssl_builder) = get_ssl_builder().ok() {
        log!("bind: 0.0.0.0:80");
        log!("bind: 0.0.0.0:{}", PORT);
        server
            //.bind(("0.0.0.0", 80))? // HTTP port
            .bind_openssl(("0.0.0.0", PORT), ssl_builder)?
    } else {
        server
    }
    .run()
    .await
}
