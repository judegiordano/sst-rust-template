use awwsy::lambda_runtime::{run, service_fn, Error, LambdaEvent};
use {{crate_name}}::{errors::AppError, logger};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct ExamplePayload {
    payload: String,
}

async fn handler(event: LambdaEvent<ExamplePayload>) -> Result<(), AppError> {
    tracing::info!("request received: {:?}", event);
    Ok(())
}

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    logger::init()?;
    run(service_fn(handler)).await
}
