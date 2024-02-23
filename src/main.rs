use std::path::Path;
use actix_files::Files;
use actix_web::{web, App, HttpServer, HttpResponse, Result};


async fn index() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("C:/Users/Cadu/ruststudy/hello_cargo/src/my-app/src/app.html")))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(Files::new("/", "../../my-app/src").index_file("app.html"))
            .service(web::resource("/").to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
