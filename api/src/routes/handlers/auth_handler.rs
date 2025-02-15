use actix_web::{post, web, HttpResponse, Responder};
use serde_json::json;
use service::{generate_token, new, validator::Validate, AppState, LoginModel, UserModel, UserResponse, UserService};

#[post("/register")]
pub async fn register(
    app_state: web::Data<AppState>,
    register_json: web::Json<UserModel>,
) -> impl Responder {
    let db = app_state.db.clone();
    let data = register_json.into_inner();

    if let Err(e) = data.validate() {
        return HttpResponse::BadRequest().json(json!(e));
    }

    match UserService::register(&db, data).await {
        Ok(active_model) => HttpResponse::Ok().json(UserResponse::from(active_model)),
        Err(_) => HttpResponse::InternalServerError().json("Failed to process request".to_string()),
    }
}

#[post("/login")]
pub async fn login(
    app_state: web::Data<AppState>,
    login_json: web::Json<LoginModel>,
) -> impl Responder {
    let db = app_state.db.clone();
    let data = login_json.into_inner();

    if let Err(e) = data.validate() {
        return HttpResponse::BadRequest().json(json!(e));
    }

    match UserService::authenticate(&db, data).await {
        Ok(active_model) => {
            let payload = new(
                active_model.email.clone().unwrap(),
                active_model.user_id.clone().unwrap(),
                24,
            );

            let token = generate_token(payload).unwrap_or_else(|_| "Error generating token".to_string());

            HttpResponse::Ok().json(json!({
                "user": UserResponse::from(active_model),
                "token": token
            }))
        }
        Err(_) => HttpResponse::InternalServerError().json("Failed to process request".to_string()),
    }
}
