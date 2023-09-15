use lambda_web::actix_web::web::{self, ServiceConfig};

mod controller;

pub fn router(cfg: &mut ServiceConfig) {
    cfg.route("/ping", web::get().to(controller::ping));
}
