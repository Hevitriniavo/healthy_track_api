use actix_web::{web, App, HttpServer};
use service::{app_state::AppState, constants };

use actix_web::middleware::Logger;
use migration::{Migrator, MigratorTrait};
use service::sea_orm::Database;

mod routes;
mod middlewares;

#[rustfmt::skip]
#[tokio::main]
pub async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    dotenvy::dotenv().ok();
    env_logger::init();
    let server_host = constants::CONFIG.server_host.clone();
    let server_port: u16 = constants::CONFIG.server_port.clone().parse().expect("Invalid port");
    let db_url = constants::CONFIG.database_url.clone();
    let conn = Database::connect(&db_url).await.unwrap();

    Migrator::up(&conn, None).await.unwrap();
    let state = AppState{db: conn.clone()};
    HttpServer::new( move || {
        let logger = Logger::default();
        App::new()
            .app_data(web::Data::new(state.clone()))
            .wrap(logger)
            .configure(routes::auth_route::config)
            .configure(routes::user_route::config)
    })
    .bind((server_host, server_port))?
    .run()
    .await
}
