use actix_cors::Cors;
use actix_web::{middleware::Logger, App, HttpServer};
use models::tokens::expire_tokens;
use std::env;
use tokio::time::Duration;
use crate::routes::user::RecoveryCodes;
use actix_web::web;

mod data {
    pub mod database;
    pub mod historic;
    pub mod product;
    pub mod user;
}
mod models {
    pub mod credentials;
    pub mod login_response;
    pub mod product;
    pub mod product_cart;
    pub mod product_cart_update;
    pub mod tokens;
    pub mod user;
}
mod routes {
    pub mod auth;
    pub mod history;
    pub mod product;
    pub mod user;
}
mod config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Criação do objeto compartilhado para armazenar os códigos de recuperação
    let recovery_codes = web::Data::new(RecoveryCodes::new());

    // Logs no terminal
    env::set_var("RUST_LOG", "actix_web=debug,my_app=info");
    env_logger::init();

    // Inicializa a verificação do token
    actix_web::rt::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(300)).await;
            expire_tokens();
        }
    });

    HttpServer::new(move || {
        App::new()
            .app_data(recovery_codes.clone())
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600),
            )
            .configure(config::config_routes)
    })
    .bind("127.0.0.1:8082")?
    .run()
    .await
}
