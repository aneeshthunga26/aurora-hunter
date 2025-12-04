use anyhow::Result;
use chrono::{DateTime, Utc};

use crate::weather::AuroraProbabilityResponse;

/// Lat(titude) and Long(itude)
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone, Copy, Hash)]
pub struct Coordinates {
    pub lat: i8,
    pub long: i16,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Location {
    pub city: String,
}

impl Location {
    pub fn from(string: &str) -> Self {
        Self { city: string.to_string() }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Colour {
    Green,
    Yellow,
    Orange,
    Red,
}

/// Stores the Aurora probability data for a given coordinates / location.
/// Colour indicates the probability where green is the lowest and red is the highest
#[derive(Debug, Clone, PartialEq)]
pub struct AuroraProbability {
    pub coordinates: Coordinates,
    pub value: u32,
    pub colour: Colour,
    pub date: DateTime<Utc>,
    pub location: Option<Location>,
}

impl AuroraProbability {
    pub fn from(data: AuroraProbabilityResponse) -> Result<Self> {
        let date = DateTime::parse_from_rfc3339(&data.date)
            .unwrap_or_else(|_| Utc::now().fixed_offset())
            .to_utc();

        let colour = match data.colour.as_str() {
            "green" => Colour::Green,
            "orange" => Colour::Orange,
            "yellow" => Colour::Yellow,
            "red" => Colour::Red,
            _ => Colour::Green,
        };

        let coordinates = Coordinates {
            lat: data.lat.parse()?,
            long: data.long.parse()?,
        };

        Ok(Self {
            coordinates,
            date,
            colour,
            value: data.value,
            location: None,
        })
    }
}
