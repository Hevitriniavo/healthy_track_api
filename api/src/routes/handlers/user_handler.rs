use actix_web::{get, web, HttpResponse, Responder};
use service::AppState;

#[get("")]
pub async fn me(
    app_state: web::Data<AppState>,
) -> impl Responder {
    let _db = app_state.db.clone();

    HttpResponse::Ok().json("Verified users".to_string())
}