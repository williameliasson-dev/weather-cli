use chrono::{self, Duration, DurationRound, Utc};
use iso8601;
use reqwest::Error;
use serde::{Deserialize, Serialize};

pub mod ascii_art;

#[derive(Debug)]
pub struct SingleData {
    pub date_time: chrono::DateTime<Utc>,
    pub temperature_2m: f32,
}
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
    pub async fn new() -> Result<WeatherData, Error> {
        let api_link = "https://api.open-meteo.com/v1/forecast?latitude=59.33&longitude=18.06&current=temperature_2m,wind_speed_10m&hourly=temperature_2m,relative_humidity_2m,wind_speed_10m";

        let weather_data: WeatherData = reqwest::Client::new()
            .get(api_link)
            .send()
            .await?
            .json()
            .await?;

        Ok(weather_data)
    }

    pub fn get_weather_4h_future(&self) -> Vec<SingleData> {
        let mut future_weather: Vec<SingleData> = Vec::new();

        let current_date_in_millis = chrono::Utc::now()
            .duration_round(Duration::hours(1))
            .unwrap()
            .timestamp_millis();

        for (i, date_time) in self.hourly.time.iter().enumerate() {
            // TODO: Make int here configurable
            if future_weather.len() > 5 {
                break;
            }

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

                let date_time = chrono::DateTime::from_timestamp(ts_secs, ts_ns as u32)
                    .expect("Error converting millis to chrono DateTime");

                future_weather.push(SingleData {
                    date_time,
                    temperature_2m: self.hourly.temperature_2m[i],
                });
            }
        }
        return future_weather;
    }
}
