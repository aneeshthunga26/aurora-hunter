use anyhow::{Result, anyhow};
use dotenvy::dotenv;
use std::sync::OnceLock;

const BASE_API_URL_KEY: &str = "BASE_API_URL";

static CELL: OnceLock<Config> = OnceLock::new();
pub struct Config {
    base_api_url: String,
}

impl Config {
    /// Get initalise and return Config
    /// panics if unable to init
    pub fn get_config() -> &'static Self {
        CELL.get_or_init(|| Self::load_from_env().unwrap())
    }

    fn load_from_env() -> Result<Self> {
        let Some(_result) = dotenv().ok() else {
            return Err(anyhow!("Failed to load .env file"));
        };

        let base_api_url = std::env::var(BASE_API_URL_KEY)?;

        Ok(Self { base_api_url })
    }

    /// Returns full api url for auroras live api
    pub fn get_auroras_api_endpoint(&self) -> String {
        self.base_api_url.clone()
    }
}
