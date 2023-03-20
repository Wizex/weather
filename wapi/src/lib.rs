//! Thin HTTP client for the [`WeatherAPI API`].
//!
//! [`WeatherAPI API`]: https://www.weatherapi.com/docs/

use anyhow::Context;
use anyhow::Result;

/// Response returned by requests.
pub type Response = reqwest::blocking::Response;

const BASE_URL: &str = "http://api.weatherapi.com/v1/history.json";
/// Get historical weather data.
///
/// # Params
///
/// `api_key` - Your unique API Key.
///
/// `date` - Timestamp (Unix time, UTC time zone).
///
/// `location` - Query parameter based on which data is sent back. See the [`WeatherAPI documentation`].
///
/// # Examples
///
/// ```
/// let response = wapi::get_historical_weather_data("London", "1586468027", "your_key");
/// ```
///
/// [`WeatherAPI documentation`]: https://www.weatherapi.com/docs/
pub fn get_historical_weather_data(location: &str, date: &str, api_key: &str) -> Result<Response> {
    let params = [("key", api_key), ("q", location), ("unixdt", date)];

    let url = reqwest::Url::parse_with_params(BASE_URL, &params)
        .with_context(|| format!("Failed to parse URL parameters from {:#?}", params))?;

    reqwest::blocking::get(url).with_context(|| "Failed to execute a request")
}
