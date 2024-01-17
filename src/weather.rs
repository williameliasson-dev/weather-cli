use chrono;
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
        let current_date_time = get_correct_date_time(&self);

        println!("{:?}", current_date_time)
    }
}

fn get_correct_date_time(weatherdata: &WeatherData) -> Result<String, ()> {
    for date_time in weatherdata.hourly.time.iter() {
        let date_string_in_iso = chrono::DateTime::parse_from_rfc3339(
            iso8601::datetime(date_time)
                .expect("Error when converting date to ISO8601")
                .to_string()
                .as_str(),
        )
        .unwrap()
        .format("%Y-%m-%d %H:%M:%S")
        .to_string();

        let current_date_in_iso = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        println!("{date_string_in_iso}");
        println!("{current_date_in_iso}");
        if date_string_in_iso == current_date_in_iso {
            return Ok(date_string_in_iso);
        }
    }

    Err(())
}
