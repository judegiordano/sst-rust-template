use axum::routing::{get, post};

mod controller;

pub fn router() -> axum::Router {
    axum::Router::new()
        .route("/ping", get(controller::ping))
        .route("/records", post(controller::create_record))
        .route("/records", get(controller::list_records))
        .route("/records/:id", get(controller::read_record))
}
