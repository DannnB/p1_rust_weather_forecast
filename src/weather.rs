/// `weather.rs` contains the definitions and implementations related to weather data structures.
/// It's used to represent and format the current weather conditions fetched from an external API.
///
/// ## Structs
///
/// - `Current`: Represents the current weather conditions including observation time, temperature, 
///   weather code, and descriptions.
/// - `Weather`: A higher-level struct that encapsulates the `Current` weather data.
///
/// ## Implementations
///
/// - `fmt::Display for Current`: Provides a custom human-readable representation of the current weather conditions.
/// - `fmt::Display for Weather`: Provides a custom human-readable representation of the weather data.
///
/// ## Dependencies
///
/// This module relies on the `serde` crate for deserializing JSON data and the `fmt` module from the standard library for custom formatting.
/// 
use std::fmt;
use serde::Deserialize;

// #[derive(Deserialize, Debug)]
// pub struct Location {
//     name: String
// }
#[derive(Deserialize)]
struct Current {
    observation_time: String,
    temperature: i32,
    weather_code: i32,
    weather_descriptions: Vec<String>,
}

impl fmt::Display for Current {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Observation Time: {}, Temperature: {}°C, Weather Code: {}, Descriptions: {:?}",
            self.observation_time, self.temperature, self.weather_code, self.weather_descriptions
        )
    }
}
#[derive(Deserialize)]
pub struct Weather {
    current: Current,
}

impl fmt::Display for Weather {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Current Weather: {}", self.current)
    }
}