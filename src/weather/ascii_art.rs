use chrono::Timelike;

use super::SingleData;

pub fn print_future_weather(singles: Vec<SingleData>) -> () {
    for weather in singles {
        let temp = weather.temperature_2m;
        let hour = weather.date_time.hour().to_string();

        let frmt: String = if hour.len() < 2 {
            String::from("H0:00").replace("H", &hour).to_string()
        } else {
            String::from("HH:00").replace("HH", &hour).to_string()
        };

        print!("|{frmt}: ");
        print!("{temp}C°");
    }
    print!("|");
    println!("");
}
