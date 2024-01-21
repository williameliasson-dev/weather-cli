use chrono::{self, Duration, DurationRound};
use iso8601;
use reqwest::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Current {
    pub time: String,
    pub temperature_2m: f32,
    pub wind_speed_10m: f32,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Hourly {
    time: Vec<String>,
    wind_speed_10m: Vec<f32>,
    temperature_2m: Vec<f32>,
    relative_humidity_2m: Vec<i32>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherData {
    pub current: Current,
    pub hourly: Hourly,
}

impl WeatherData {
    pub async fn construct_weather_data() -> Result<WeatherData, Error> {
        let api_link = "https://api.open-meteo.com/v1/forecast?latitude=59.33&longitude=18.06&current=temperature_2m,wind_speed_10m&hourly=temperature_2m,relative_humidity_2m,wind_speed_10m";

        let weather_data: WeatherData = reqwest::Client::new()
            .get(api_link)
            .send()
            .await?
            .json()
            .await?;

        Ok(weather_data)
    }

    pub fn get_weather_4h_future(&self) {
        let current_date_time = get_future_weather(&self);

        println!("{:?}", current_date_time)
    }
}

fn get_future_weather(weatherdata: &WeatherData) -> Result<String, ()> {
    let current_date_in_millis = chrono::Utc::now()
        .duration_round(Duration::hours(1))
        .unwrap()
        .timestamp_millis();

    for date_time in &weatherdata.hourly.time {
        let date_in_millis = chrono::DateTime::parse_from_rfc3339(
            iso8601::datetime(&date_time.to_owned())
                .expect("Error when converting date to ISO8601")
                .to_string()
                .as_str(),
        )
        .unwrap()
        .timestamp_millis();

        if date_in_millis >= current_date_in_millis {
            let ts_secs = date_in_millis / 1000;
            let ts_ns = (date_in_millis % 1000) * 1_000_000;

            println!(
                "{:?}",
                chrono::DateTime::from_timestamp(ts_secs, ts_ns as u32)
                    .unwrap()
                    .to_rfc3339()
                    .to_string()
            );
        }
    }

    Err(())
}
