use reqwest::Error;
use std::{env, usize};
use tokio;
use weather::WeatherData;

mod weather;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<_> = env::args().collect();

    if args.len() > 1 {
        let arg = args[1].as_str();

        match arg {
            "future" => {
                if args.len() > 2 {
                    let hours: usize = args[2].parse::<usize>().expect("Invalid parameters");

                    print_future_weather(hours).await?
                }
            }
            "current" => (),
            _ => println!("Unknown arguments"),
        }
    }

    Ok(())
}

async fn print_future_weather(hours: usize) -> Result<(), Error> {
    let weather_data = WeatherData::new().await?;

    let future_weathers = weather_data.get_weather_future(hours);

    weather::ascii_art::print_future_weather(future_weathers);

    Ok(())
}
