use reqwest::Client;
use serde::Deserialize;
use std::fmt::Display;

use crate::config::Config;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Location {
    key: String,
    localized_name: String,
    country: Country,
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.localized_name, self.country.id)
    }
}

#[derive(Deserialize, Debug)]
pub struct Country {
    #[serde(alias = "ID")]
    pub id: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Forecast {
    pub headline: Headline,
}

#[derive(Deserialize, Debug)]
pub struct Headline {
    #[serde(alias = "Text")]
    pub overview: String,
}

#[derive(Debug)]
pub struct CouldNotFindLocation {
    place: String,
}

impl Display for CouldNotFindLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Could not find location '{}'", self.place)
    }
}

impl std::error::Error for CouldNotFindLocation {}

/// Get Weather forecast for a given Location
pub async fn get_forecast(
    place: &str,
    api_key: &str,
    client: &Client,
) -> Result<(Location, Forecast), Box<dyn std::error::Error>> {
    let config = Config::get_config();

    let locations_url = config.get_locations_api_url();
    let forecasts_url = config.get_forecasts_api_url();

    let url = format!("{locations_url}?apikey={api_key}&q={place}");
    let request = client.get(url).build().unwrap();

    let resp = client
        .execute(request)
        .await?
        .json::<Vec<Location>>()
        .await?;

    let first_location = resp
        .into_iter()
        .next()
        .ok_or_else(|| CouldNotFindLocation {
            place: place.to_owned(),
        })?;

    let url = format!("{}{}?apikey={}", forecasts_url, first_location.key, api_key);

    let request = client.get(url).build().unwrap();
    let forecast = client.execute(request).await?.json::<Forecast>().await?;

    Ok((first_location, forecast))
}
