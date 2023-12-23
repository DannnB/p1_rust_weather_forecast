/// `main.rs` is the entry point of the application. 
/// It defines the `Current` and `Weather` structs along with their implementations.
///
/// ## Structs
///
/// - `Current`: Holds data about the current weather observation including time, temperature, 
///   weather code, and descriptions.
/// - `Weather`: Encapsulates the current weather details.
///
/// ## Implementations
///
/// - `fmt::Display for Current`: Provides a human-readable representation of the `Current` struct.
/// - `fmt::Display for Weather`: Provides a human-readable representation of the `Weather` struct.
///
/// ## Dependencies
///
/// This file requires the `serde` crate for deserializing JSON data into Rust structs and 
/// the `fmt` module from the standard library for formatting the output.

mod custom_error;
mod weather;

use std::env;
use dotenv::dotenv;
use weather::Weather;

use reqwest;

fn main() {
    // Initialize the dotenv library to read from the `.env` file
    dotenv().ok();

    let api_key = match env::var("WEATHER_API_KEY") {
        Ok(key) => key,
        Err(_) => panic!("WEATHER_API_KEY not set in .env or as an environment variable."),
    };

    println!("This is the Weather Forecast CLI");
    let url: String = format!("http://api.weatherstack.com/current?access_key={}&query=Worcester UK",
    api_key);

    match fetch_weather(&url) {
        Ok(weather) => {
            println!("Current Weather:");
            println!("{}", weather);
        }
        Err(e) => eprintln!("Failed to fetch weather: {}", e),
    
    }
}

pub fn fetch_weather(url: &str) -> Result<Weather, custom_error::CustomError> {
    let response: String = reqwest::blocking::get(url)?.text()?;
    let current_weather: Weather = serde_json::from_str(&response)?;

    // println!("Response: {:?}", &response);

    Ok(current_weather)
}