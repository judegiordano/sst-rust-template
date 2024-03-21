pub mod controllers;
pub mod models;

pub mod config {
    use serde::{Deserialize, Serialize};
    use tracing::Level;

    use crate::errors::AppError;

    #[derive(Debug, Deserialize, Serialize)]
    pub enum Stage {
        Local,
        Test,
        Prod,
        Other(String),
    }

    #[derive(Debug)]
    pub struct Env {
        pub stage: Stage,
        pub log_level: Level,
        pub mongo_uri: String,
    }

    impl Env {
        fn _get_required_string(key: &str) -> String {
            match std::env::var(key.trim().to_uppercase()) {
                Ok(value) => value,
                Err(err) => {
                    eprintln!("{key} not found: {err}");
                    std::process::exit(1);
                }
            }
        }

        pub fn new() -> Result<Self, AppError> {
            if cfg!(debug_assertions) {
                use dotenv::dotenv;
                dotenv().ok();
            }
            let env = Self {
                stage: match Self::_get_required_string("STAGE").to_uppercase().as_str() {
                    "LOCAL" => Stage::Local,
                    "PROD" => Stage::Prod,
                    "TEST" => Stage::Test,
                    other => Stage::Other(other.to_string()),
                },
                log_level: match Self::_get_required_string("LOG_LEVEL")
                    .to_uppercase()
                    .as_str()
                {
                    "DEBUG" => Level::DEBUG,
                    "INFO" => Level::INFO,
                    "WARN" => Level::WARN,
                    _ => Level::ERROR,
                },
                mongo_uri: Self::_get_required_string("MONGO_URI"),
            };
            Ok(env)
        }
    }
}

pub mod errors {
    use axum::{
        http::StatusCode,
        response::{IntoResponse, Response},
        Json,
    };
    use serde::Serialize;
    use thiserror::Error;

    #[derive(Debug, Error)]
    pub enum AppError {
        #[error("environment error: {0}")]
        EnvError(String),
        #[error("internal server error: {0}")]
        InternalServerError(String),
        #[error("unauthorized: {0}")]
        Unauthorized(String),
        #[error("bad request: {0}")]
        BadRequest(String),
        #[error("not found: {0}")]
        NotFound(String),
    }

    #[allow(clippy::needless_pass_by_value)]
    impl AppError {
        pub fn env_error(error: impl ToString) -> Self {
            Self::EnvError(error.to_string())
        }

        pub fn unauthorized(error: impl ToString) -> Self {
            Self::Unauthorized(error.to_string())
        }

        pub fn not_found(error: impl ToString) -> Self {
            Self::NotFound(error.to_string())
        }

        pub fn internal_server_error(error: impl ToString) -> Self {
            Self::InternalServerError(error.to_string())
        }

        pub fn bad_request(error: impl ToString) -> Self {
            Self::BadRequest(error.to_string())
        }
    }

    #[derive(Serialize)]
    pub struct ErrorMessage {
        pub error: String,
    }

    impl IntoResponse for AppError {
        fn into_response(self) -> Response {
            let status = match self {
                Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
                Self::NotFound(_) => StatusCode::NOT_FOUND,
                Self::BadRequest(_) => StatusCode::BAD_REQUEST,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            tracing::error!("[ERROR]: {self:?}");
            let error = ErrorMessage {
                error: self.to_string(),
            };
            (status, Json(error)).into_response()
        }
    }
}

pub mod logger {
    use tracing::Level;
    use tracing_subscriber::FmtSubscriber;

    use crate::{config, errors::AppError};

    pub fn init() -> Result<(), AppError> {
        let log_level = config::Env::new().map_or(Level::ERROR, |e| e.log_level);
        let subscriber = FmtSubscriber::builder().with_max_level(log_level).finish();
        tracing::subscriber::set_global_default(subscriber)
            .map_err(AppError::internal_server_error)?;
        Ok(())
    }
}
