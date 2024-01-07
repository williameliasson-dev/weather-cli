use reqwest::Error;
use serde::{Deserialize, Serialize};
use tokio;

#[derive(Debug, Serialize, Deserialize)]
struct Current {
    time: String,
    temperature_2m: f32,
    wind_speed_10m: f32,
}
#[derive(Debug, Serialize, Deserialize)]
struct Hourly {
    time: Vec<String>,
    wind_speed_10m: Vec<f32>,
    temperature_2m: Vec<f32>,
    relative_humidity_2m: Vec<i32>,
}
#[derive(Debug, Serialize, Deserialize)]
struct WeatherData {
    current: Current,
    hourly: Hourly,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let weather_data = get_weather_data().await?;
    println!("{:?}", weather_data);
    Ok(())
}

async fn get_weather_data() -> Result<WeatherData, Error> {
    let api_link = "https://api.open-meteo.com/v1/forecast?latitude=52.52&longitude=13.41&current=temperature_2m,wind_speed_10m&hourly=temperature_2m,relative_humidity_2m,wind_speed_10m";

    let weather_data: WeatherData = reqwest::Client::new()
        .get(api_link)
        .send()
        .await?
        .json()
        .await?;

    Ok(weather_data)
}