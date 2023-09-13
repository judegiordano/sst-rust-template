use anyhow::Result;
use lambda_web::{
    actix_web::{web::scope, App},
    run_actix_on_lambda,
};
use sst_example::controllers::routes;

#[tokio::main]
pub async fn main() -> Result<(), lambda_http::Error> {
    let factory = move || App::new().service(scope("/api").configure(routes));
    run_actix_on_lambda(factory).await?;
    Ok(())
}
