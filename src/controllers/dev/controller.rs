use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use mongoose::{doc, types::ListOptions, DateTime, Model};
use serde::Serialize;

use crate::{
    config::{Env, Stage},
    errors::AppError,
    models::record::Record,
};

#[derive(Serialize)]
struct PingMessage {
    stage: Stage,
}

pub async fn ping() -> Result<impl IntoResponse, AppError> {
    let Env { stage, .. } = Env::new().map_err(AppError::env_error)?;
    Ok(Json(PingMessage { stage }))
}

pub async fn create_record() -> Result<impl IntoResponse, AppError> {
    let now = DateTime::now();
    let new_record = Record {
        payload: format!("request received: {now}"),
        ..Default::default()
    };
    Ok((StatusCode::CREATED, Json(new_record.save().await)))
}

pub async fn read_record(Path(id): Path<String>) -> Result<impl IntoResponse, AppError> {
    let record = Record::read_by_id(&id).await.map_err(AppError::not_found)?;
    Ok(Json(record))
}

pub async fn list_records() -> Result<impl IntoResponse, AppError> {
    let records = Record::list(
        Default::default(),
        ListOptions {
            sort: doc! { "created_at": -1 },
            ..Default::default()
        },
    )
    .await
    .map_err(AppError::not_found)?;
    Ok(Json(records))
}
