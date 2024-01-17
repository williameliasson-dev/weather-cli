use reqwest::Error;
use tokio;
use weather::WeatherData;

mod weather;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let weather_data = WeatherData::construct_weather_data().await?;

    println!("{:?}°C", weather_data.current.temperature_2m);
    println!("{:?}m/s", weather_data.current.wind_speed_10m);
    println!("time: {:?}", weather_data.current.time);

    weather_data.get_weather_4h_future();

    Ok(())
}
