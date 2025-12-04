use chrono::{TimeDelta, Utc};
use reqwest::Client;
use serde::Deserialize;

use crate::{
    config::Config,
    store::Store,
    types::{AuroraProbability, Coordinates},
};

#[derive(Debug, Deserialize)]
pub struct CalculatedAuroraProbability {
    pub lat: f32,
    pub long: f32,
    pub value: u32,
    pub colour: String,
}

#[derive(Debug, Deserialize)]
pub struct AuroraProbabilityResponse {
    pub date: String,
    pub calculated: CalculatedAuroraProbability,
    pub colour: String,
    pub lat: String,
    pub long: String,
    pub value: u32,
    pub message: Vec<String>,
}

/// Get Aurora prediction for a given Location
pub async fn get_aurora_prediction(
    coordinates: Coordinates,
    client: &Client,
) -> Result<AuroraProbability, Box<dyn std::error::Error>> {
    let config = Config::get_config();
    let store = Store::get_store().await;

    // Check if there exists an entry in the store for the requested coordinates
    let forecast = store.get_probability_data_by_coordinates(coordinates).await;
    if let Some(forecast) = forecast {
        // Check if it was updated less than 10 mins ago
        if Utc::now() - forecast.date > TimeDelta::minutes(10) {
            // Return the cached data
            return Ok(forecast);
        }
    }

    let auroras_url = config.get_auroras_api_endpoint();

    let url = format!(
        "{auroras_url}?type=ace&data=probability&lat={}&long={}",
        coordinates.lat, coordinates.long
    );
    let request = client.get(url).build()?;

    let resp = client
        .execute(request)
        .await?
        .json::<AuroraProbabilityResponse>()
        .await?;

    let aurora_probability = AuroraProbability::from(resp)?;

    // Store the response in cache
    store.add_probability_data(aurora_probability.clone()).await;

    Ok(aurora_probability)
}
