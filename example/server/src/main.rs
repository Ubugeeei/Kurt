use actix_web::{get, App, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    let html = std::fs::read_to_string("static/index.html").unwrap();
    html
}

#[get("/scroll")]
async fn scroll() -> impl Responder {
    let html = std::fs::read_to_string("static/scroll.html").unwrap();
    html
}

#[get("/js")]
async fn js() -> impl Responder {
    let html = std::fs::read_to_string("static/js.html").unwrap();
    html
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // .route("/hello", web::get().to(|| async { "Hello World!" }))
            .service(index)
            .service(scroll)
            .service(js)
        })
    .bind(("localhost", 3000))?
    .run()
    .await
}