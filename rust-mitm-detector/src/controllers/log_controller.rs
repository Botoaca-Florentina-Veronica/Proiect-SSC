// === src/controllers/log_controller.rs ===
use crate::models::log::LogEntry;
use mongodb::{Database, bson::doc};
use chrono::Utc;
use axum::{Json, extract::State};
use std::sync::Arc;
use serde_json::json;

pub async fn log_packet(
    db: &Database,
    source_ip: String,
    dest_ip: String,
    source_mac: String,
    dest_mac: String,
    protocol: String,
    suspicious: bool,
) {
    let collection = db.collection::<LogEntry>("logs");

    let entry = LogEntry {
        source_ip,
        dest_ip,
        source_mac,
        dest_mac,
        protocol,
        suspicious,
        timestamp: Utc::now(),
    };

    match collection.insert_one(entry, None).await {
        Ok(_) => log::info!("Log salvat."),
        Err(e) => log::error!("Eroare salvare log: {}", e),
    }
}

pub async fn get_logs(State(db): State<Arc<Database>>) -> Json<serde_json::Value> {
    let collection = db.collection::<LogEntry>("logs");
    let cursor = collection.find(None, None).await.expect("Nu pot citi logs");

    let results: Vec<_> = cursor
        .filter_map(|doc| async { doc.ok() })
        .collect()
        .await;

    Json(json!({ "logs": results }))
}
