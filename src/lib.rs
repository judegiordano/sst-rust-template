pub mod controllers;

pub mod config {
    use std::env::VarError;

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

    pub struct Env {
        pub stage: Stage,
        pub region: String,
        pub log_level: Level,
        pub mongo_uri: String,
    }

    impl Env {
        pub fn _init() -> Result<Self, VarError> {
            Ok(Self {
                stage: match std::env::var("STAGE")?.to_uppercase().as_str() {
                    "LOCAL" => Stage::Local,
                    "PROD" => Stage::Prod,
                    "TEST" => Stage::Test,
                    other => Stage::Other(other.to_string()),
                },
                region: std::env::var("REGION")?,
                log_level: match std::env::var("LOG_LEVEL")?.to_uppercase().as_str() {
                    "DEBUG" => Level::DEBUG,
                    "INFO" => Level::INFO,
                    "WARN" => Level::WARN,
                    "ERROR" => Level::ERROR,
                    _ => Level::ERROR,
                },
                mongo_uri: std::env::var("MONGO_URI")?,
            })
        }

        pub fn new() -> Result<Self, AppError> {
            match Self::_init() {
                Ok(env) => Ok(env),
                Err(err) => Err(AppError::InternalServerError {
                    error: Some(err.to_string()),
                }),
            }
        }
    }
}

pub mod errors {
    use std::fmt::Display;

    use lambda_http::http::{HeaderValue, StatusCode};
    use lambda_web::actix_web::{body, error, http, HttpResponse};
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum AppError {
        #[error("{error:?}")]
        Unauthorized { error: Option<String> },
        #[error("{error:?}")]
        InternalServerError { error: Option<String> },
        #[error("{error:?}")]
        NotFound { error: Option<String> },
    }

    impl error::ResponseError for AppError {
        fn status_code(&self) -> StatusCode {
            match self {
                AppError::Unauthorized { error: _ } => StatusCode::UNAUTHORIZED,
                AppError::InternalServerError { error: _ } => StatusCode::INTERNAL_SERVER_ERROR,
                AppError::NotFound { error: _ } => StatusCode::NOT_FOUND,
            }
        }

        fn error_response(&self) -> HttpResponse<body::BoxBody> {
            tracing::error!("[ERROR]: {self:?}");
            let mut res = HttpResponse::new(self.status_code());
            res.headers_mut().insert(
                http::header::CONTENT_TYPE,
                HeaderValue::from_static("application/json"),
            );
            let message = match self {
                AppError::Unauthorized { error } => error
                    .as_ref()
                    .map_or("unauthorized".to_string(), |e| e.to_string()),
                AppError::InternalServerError { error } => error
                    .as_ref()
                    .map_or("internal server error".to_string(), |e| e.to_string()),
                AppError::NotFound { error } => error
                    .as_ref()
                    .map_or("not found".to_string(), |e| e.to_string()),
            };
            let error_raw = format!(r#"{{"error":"{message}"}}"#);
            res.set_body(body::BoxBody::new(error_raw))
        }
    }

    pub type AppResponse = Result<HttpResponse, AppError>;

    // THIS IS FOR MORE GENERIC ERRORS WITH ANYHOW
    #[derive(Debug)]
    pub struct ApiError(anyhow::Error);
    pub type ApiResponse = Result<HttpResponse, ApiError>;

    impl<E> From<E> for ApiError
    where
        E: Into<anyhow::Error>,
    {
        fn from(err: E) -> Self {
            Self(err.into())
        }
    }

    impl Display for ApiError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl error::ResponseError for ApiError {
        fn status_code(&self) -> StatusCode {
            StatusCode::INTERNAL_SERVER_ERROR
        }

        fn error_response(&self) -> HttpResponse<body::BoxBody> {
            tracing::error!("[ERROR]: {self:?}");
            let mut res = HttpResponse::new(self.status_code());
            res.headers_mut().insert(
                http::header::CONTENT_TYPE,
                HeaderValue::from_static("application/json"),
            );
            let error_raw = format!(r#"{{"error":"{self}"}}"#);
            res.set_body(body::BoxBody::new(error_raw))
        }
    }
}

pub mod logger {
    use tracing::Level;

    use crate::config;

    pub fn init() {
        let log_level = config::Env::new().map_or(Level::ERROR, |e| e.log_level);
        tracing_subscriber::fmt()
            .with_max_level(log_level)
            .with_target(false)
            .without_time()
            .init()
    }
}
