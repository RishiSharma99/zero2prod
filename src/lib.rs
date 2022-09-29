use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};


#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[get("/health_checkup")]
async fn health_checkup() -> impl Responder {
    HttpResponse::Ok()
}

pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/hello", web::get().to(|| async { "Hello World!" }))
            .service(greet)
            .service(health_checkup)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


