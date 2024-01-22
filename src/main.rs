use reqwest::Error;
use tokio;
use weather::WeatherData;

mod weather;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let weather_data = WeatherData::new().await?;

    let future_weathers = weather_data.get_weather_4h_future();

    println!("{:?}", future_weathers);

    Ok(())
}
