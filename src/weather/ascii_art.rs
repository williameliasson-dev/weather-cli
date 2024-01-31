use chrono::Timelike;

use super::SingleData;

pub fn print_future_weather(singles: Vec<SingleData>) -> () {
    for (i, weather) in singles.iter().enumerate() {
        let temp = weather.temperature_2m;
        let hour = weather.date_time.hour();

        print!("|{hour}: ");
        print!("{temp}C°");
    }
    print!("|");
    println!("");
}
