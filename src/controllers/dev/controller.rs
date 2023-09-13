use lambda_web::actix_web::HttpResponse;
use serde::Serialize;

#[derive(Serialize)]
struct PingMessage {
    message: String,
}

pub async fn ping() -> HttpResponse {
    HttpResponse::Ok().json(PingMessage {
        message: "pong".to_string(),
    })
}
