use anyhow::Result;
use lambda_web::{
    actix_web::{web::scope, App},
    run_actix_on_lambda as run,
};
use sst_example::controllers::routes;

#[tokio::main]
pub async fn main() -> Result<(), lambda_http::Error> {
    run(move || App::new().service(scope("/api").configure(routes))).await
}
