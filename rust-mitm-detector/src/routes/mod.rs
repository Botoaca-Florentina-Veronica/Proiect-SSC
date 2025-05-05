// === src/routes/mod.rs ===
pub fn routes(db: mongodb::Database) -> axum::Router {
    use axum::{routing::get, Router};
    use std::sync::Arc;
    use crate::controllers::log_controller::get_logs;

    let shared_db = Arc::new(db);

    Router::new()
        .route("/logs", get(get_logs))
        .with_state(shared_db)
}
