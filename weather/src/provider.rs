use std::fmt;

use anyhow::Context;
use serde_derive::{Deserialize, Serialize};

#[derive(Copy, Clone, PartialEq, Eq, clap::ValueEnum, Serialize, Deserialize, Hash)]
pub enum Provider {
    #[serde(rename = "open-weather")]
    OpenWeather,

    #[serde(rename = "weather-api")]
    WeatherApi,
}

impl fmt::Display for Provider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

#[derive(Serialize, Deserialize)]
struct Coordinates {
    #[serde(rename = "lat")]
    latitude: f64,

    #[serde(rename = "lon")]
    longitude: f64,
}

impl Provider {
    pub fn get_weather(&self, api_key: &str, address: &str, date: &str) -> anyhow::Result<String> {
        match self {
            Provider::OpenWeather => {
                let response = owm::geocoding::get_coordinates_by_location(address, api_key, 1)
                    .with_context(|| format!("Failed to get coordinates for {address}"))?;

                let body = response
                    .text()
                    .with_context(|| "Failed to get coordinates")?;

                let coordinates: Vec<Coordinates> = serde_json::from_str(&body)
                    .with_context(|| format!("Failed to get coordinates. Response: {body}"))?;
                let coordinates = coordinates.into_iter().next().ok_or_else(|| {
                    anyhow::anyhow!(
                    "No coordinates for specified address. Probably, specified location is wrong"
                )
                })?;

                let response = owm::one_call::get_historical_weather_data(
                    (
                        coordinates.latitude.to_string(),
                        coordinates.longitude.to_string(),
                    ),
                    date,
                    api_key,
                )
                .with_context(|| "Failed to get weather")?;

                response.text().with_context(|| "Failed to parse response")
            }
            Provider::WeatherApi => {
                let response = wapi::get_historical_weather_data(address, date, api_key)
                    .with_context(|| "Failed to get weather")?;
                response.text().with_context(|| "Failed to parse response")
            }
        }
    }
}
