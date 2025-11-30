use chrono::{DateTime, Utc};

/// Lat(titude) and Long(itude)
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone, Copy)]
pub struct Coordinates {
    pub lat: i8,
    pub long: i16,
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
    pub value: f64,
    pub colour: Colour,
    pub date: DateTime<Utc>,
    pub location_name: Option<String>,
}