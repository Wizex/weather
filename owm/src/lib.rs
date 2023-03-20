//! Thin HTTP client for [`OpenWeatherMap API`].
//!
//! [`OpenWeatherMap API`]: https://openweathermap.org/api

/// Response returned by requests.
pub type Response = reqwest::blocking::Response;

/// Contains requests for [`One Call API 3.0`].
///
/// [`One Call API 3.0`]: https://openweathermap.org/api/one-call-3
pub mod one_call {
    use super::Response;
    use anyhow::Context;
    use anyhow::Result;

    const BASE_URL: &str = "https://api.openweathermap.org/data/3.0/onecall/timemachine";

    pub type Latitude = String;
    pub type Longitude = String;
    pub type GeoCoordinates = (Latitude, Longitude);

    /// Get historical weather data.
    ///
    /// It mostly is used along with the [`Geocoding API`].
    /// See it's documentation for more details.
    ///
    /// # Params
    ///
    /// `coordinates` - Geographical coordinates (latitude, longitude).
    ///
    /// `date` - Timestamp (Unix time, UTC time zone), e.g. dt=1586468027. Data is available from January 1st, 1979.
    ///
    /// `api_key` - Your unique API key.
    ///
    /// # Examples
    ///
    /// ```
    /// let coordinates = ("39.099724".to_string(), "-94.57833".to_string());
    /// let date = "1586468027";
    /// let api_key = "your_key";
    /// let response = owm::one_call::get_historical_weather_data(coordinates, date, api_key);
    /// ```
    ///
    /// [`Geocoding API`]: https://openweathermap.org/api/geocoding-api
    pub fn get_historical_weather_data(
        coordinates: GeoCoordinates,
        date: &str,
        api_key: &str,
    ) -> Result<Response> {
        let params = [
            ("lat", coordinates.0.as_str()),
            ("lon", coordinates.1.as_str()),
            ("dt", date),
            ("appid", api_key),
        ];

        let url = reqwest::Url::parse_with_params(BASE_URL, &params)
            .with_context(|| format!("Failed to parse URL parameters from {:#?}", params))?;

        reqwest::blocking::get(url).with_context(|| "Failed to execute a request")
    }
}

/// Contains requests for [`Geocoding API`].
///
/// [`Geocoding API`]: https://openweathermap.org/api/geocoding-api
pub mod geocoding {
    use super::Response;
    use anyhow::Context;
    use anyhow::Result;

    const BASE_URL: &str = "http://api.openweathermap.org/geo/1.0/direct";

    /// Get coordinates by location.
    ///
    /// # Params
    ///
    /// `location` - city name, state code (only for the US) and country code divided by comma. Please use ISO 3166 country codes.
    ///
    /// `api_key` - Your unique API key.
    ///
    /// `limit` - Number of the locations in the API response (up to 5 results can be returned in the API response).
    ///
    /// See the [`Geocoding API documentation`] for more details.
    ///
    /// # Examples
    ///
    /// ```
    /// let response = owm::geocoding::get_coordinates_by_location("London", "your_key", 1);
    /// ```
    ///
    /// [`Geocoding API documentation`]: https://openweathermap.org/api/geocoding-api
    pub fn get_coordinates_by_location(
        location: &str,
        api_key: &str,
        limit: i32,
    ) -> Result<Response> {
        let params = [
            ("q", location),
            ("appid", api_key),
            ("limit", &limit.to_string()),
        ];

        let url = reqwest::Url::parse_with_params(BASE_URL, &params)
            .with_context(|| format!("Failed to parse URL parameters from {:#?}", params))?;

        reqwest::blocking::get(url).with_context(|| "Failed to execute a request")
    }
}
