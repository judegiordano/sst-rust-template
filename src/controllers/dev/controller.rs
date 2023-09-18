use lambda_web::actix_web::HttpResponse;
use serde::Serialize;

use crate::{
    config::{self, Env, Stage},
    errors::ApiResponse,
};

#[derive(Serialize)]
struct PingMessage {
    stage: Stage,
    region: String,
}

pub async fn ping() -> ApiResponse {
    let Env { stage, region } = config::Env::new()?;
    Ok(HttpResponse::Ok().json(PingMessage { stage, region }))
}
