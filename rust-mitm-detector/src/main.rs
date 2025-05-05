// === src/main.rs ===
use dotenv::dotenv;
use std::env;
use crate::network_monitor::start_sniffer;
use crate::routes::routes;
use axum::Router;
use std::net::SocketAddr;
use tokio::task;
use axum::Server;


mod config;
mod controllers;
mod middleware;
mod models;
mod network_monitor;
mod routes;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let db = config::mongo::init().await.expect("Eșec conectare MongoDB");
    let db_clone = db.clone();

    // Pornește monitorul de rețea pe un thread separat
    task::spawn(async move {
        start_sniffer(db_clone).await;
    });

    // Pornește serverul REST
    let app = routes(db);
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Server REST la http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
