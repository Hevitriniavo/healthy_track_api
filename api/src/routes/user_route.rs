use super::handlers;
use crate::middlewares::AuthMiddleware;
use actix_web::middleware::from_fn;
use actix_web::web;
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .wrap(from_fn(AuthMiddleware::check))
            .service(handlers::user_handler::me)
    );
}