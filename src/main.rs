// use serde::{Serialize, Deserialize};

mod structs;
mod read_env;

use read_env::get_apikey;
use structs::ApiResponse;
use colored::Colorize;

const OPEN_WEATHER_URL: &str = "https://api.openweathermap.org/data/2.5";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get env file content
    let api_key = get_apikey("OPEN_WEATHER".to_string());

    let city = "lyon".to_string();

    let url = format!("{}/weather?q={}&appId={}", OPEN_WEATHER_URL, city, api_key);

    let body: structs::ApiResponse = reqwest::get(url).await?
        .json().await?;

    let res = print_weather(body);

    Ok(())
}

fn print_weather (current: ApiResponse) {
    println!(
        r#"
        |{}| {}
        |{}| {}
        |{}| {}
        |{}| lon {}, lat {}
        "#,
        green("City"),
        // format!("City").bold().green(),
        current.name,
        green("Weather"),
        &current.weather.first().unwrap().description,
        green("Temperature"),
        current.main.temp,
        green("Coordinates"),
        current.coord.lon,
        current.coord.lat,
    );

    fn green (name: &str) -> String {
        format!("{}", name).bold().green().to_string()
    }
}