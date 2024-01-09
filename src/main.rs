use reqwest::Error;
use tokio;
use weather::WeatherData;

mod weather;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let weather_data = WeatherData::construct_weather_data().await?;

    println!("{:?}", weather_data.current.temperature_2m);

    Ok(())
}
