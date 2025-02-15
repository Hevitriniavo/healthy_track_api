use super::handlers;
use actix_web::web;
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(handlers::auth_handler::register)
            .service(handlers::auth_handler::login)
    );
}