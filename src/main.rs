mod api;
mod cli;
mod db;
mod auth;
mod websocket;
mod models;

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::env;
use tokio::sync::mpsc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_pool = db::init_db().await.expect("Falha ao conectar ao banco de dados");

    let (tx, rx) = mpsc::channel(32);
    websocket::start_listener(rx);

    let server = HttpServer::new(move || {
        App::new()
            .app_data(db_pool.clone())
            .configure(api::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run();

    cli::start(db_pool.clone(), tx).await;
    server.await
}
