use anyhow::Result;
use controllers::routes;
use lambda_web::{
    actix_web::{web::scope, App},
    run_actix_on_lambda,
};
pub mod controllers;

#[tokio::main]
pub async fn main() -> Result<(), lambda_http::Error> {
    let factory = move || App::new().service(scope("/api").configure(routes));
    run_actix_on_lambda(factory).await?;
    Ok(())
}
