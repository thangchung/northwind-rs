use actix_web::http::{StatusCode};
use actix_web::{get, App, HttpServer, HttpRequest, HttpResponse, Result};

#[get("/")]
async fn index(req: HttpRequest) -> Result<HttpResponse> {
    println!("{:?}", req);

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/welcome.html")))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");

    HttpServer::new(|| { 
        App::new()
            .service(index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
