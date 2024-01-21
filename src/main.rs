use reqwest::Error;
use tokio;
use weather::WeatherData;

mod weather;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let weather_data = WeatherData::construct_weather_data().await?;

    weather_data.get_weather_4h_future();

    Ok(())
}
