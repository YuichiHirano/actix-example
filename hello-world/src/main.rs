use actix_web::{App, HttpServer};
mod index;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(index::AppState {
                app_name: String::from("Actix-web"),
            })
            .service(index::index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
