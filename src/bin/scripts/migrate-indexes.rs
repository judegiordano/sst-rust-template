use {{crate_name}}::{errors::AppError, logger, models::record::Record};

#[tokio::main]
pub async fn main() -> Result<(), AppError> {
    logger::init()?;
    let results = futures::try_join!(Record::migrate())?;
    tracing::info!("{:#?}", results);
    Ok(())
}
