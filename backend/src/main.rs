use actix_files::Files;
use actix_web::{get, middleware, post, web, App, HttpResponse, HttpServer, Responder};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

const SOCKET_ADDRESS: &str = "0.0.0.0:33080";

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut ssl_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;
    ssl_builder.set_private_key_file("privkey16.pem", SslFiletype::PEM)?;
    ssl_builder.set_certificate_chain_file("fullchain16.pem")?;

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .service(hello)
            .service(echo)
            .service(Files::new("/", "./dist/").index_file("index.html"))
            .default_service(
                //web::route().to(|| HttpResponse::Found().header("Location", "/").finish()),
                //web::to(|| HttpResponse::Found().insert_header(("Location", "/"))),
                web::to(|| HttpResponse::NotFound()),
            )
    })
    .bind("127.0.0.1:8080")?
    .bind_openssl(SOCKET_ADDRESS, ssl_builder)?
    .run()
    .await
}
