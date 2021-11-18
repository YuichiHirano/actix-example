use actix_web::{App,web, HttpServer};
use std::sync::Mutex;
mod index;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(index::AppStateWithCounter{
        counter: Mutex::new(0),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(counter.clone()).route("/", web::get().to(index::index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
