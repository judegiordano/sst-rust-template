pub mod controllers;

pub mod config {
    use std::env::VarError;

    pub enum Stage {
        Local,
        Test,
        Prod,
        Other(String),
    }

    pub struct Env {
        pub stage: Stage,
        pub region: String,
    }

    impl Env {
        pub fn new() -> Result<Self, VarError> {
            Ok(Self {
                stage: match std::env::var("STAGE")?.to_uppercase().as_str() {
                    "LOCAL" => Stage::Local,
                    "PROD" => Stage::Prod,
                    "TEST" => Stage::Test,
                    other => Stage::Other(other.to_string()),
                },
                region: std::env::var("REGION")?,
            })
        }
    }
}
