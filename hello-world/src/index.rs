use actix_web::{get, web};

// This struct represents state
pub struct AppState {
    pub app_name: String,
}

#[get("/")]
pub async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // <- get app_name

    format!("Hello {}!", app_name) // <- response with app_name
}
