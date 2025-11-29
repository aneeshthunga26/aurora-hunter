use anyhow::{Result, anyhow};
use dotenvy::dotenv;
use std::sync::OnceLock;

const BASE_API_URL_KEY: &str = "BASE_API_URL";
const LOCATIONS_API_ENDPOINT_KEY: &str = "LOCATIONS_API_ENDPOINT";
const FORECASTS_API_ENDPOINT_KEY: &str = "FORECASTS_API_ENDPOINT";

static CELL: OnceLock<Config> = OnceLock::new();
pub struct Config {
    base_api_url: String,
    locations_api_endpoint: String,
    forecasts_api_endpoint: String,
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
        let locations_api_endpoint = std::env::var(LOCATIONS_API_ENDPOINT_KEY)?;
        let forecasts_api_endpoint = std::env::var(FORECASTS_API_ENDPOINT_KEY)?;

        Ok(Self {
            base_api_url,
            locations_api_endpoint,
            forecasts_api_endpoint,
        })
    }

    /// Returns full api url for locations endpoint
    pub fn get_locations_api_url(&self) -> String {
        self.base_api_url.clone() + &self.locations_api_endpoint
    }

    /// Returns full api url for forecasts endpoint
    pub fn get_forecasts_api_url(&self) -> String {
        self.base_api_url.clone() + &self.forecasts_api_endpoint
    }
}
