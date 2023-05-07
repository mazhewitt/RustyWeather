use serde::Deserialize;
use esp_idf_svc::http::{Client, Connection};
use std::str::from_utf8;

#[derive(Deserialize, Debug)]
pub struct WeatherData {
    pub weather: Vec<Weather>,
    pub main: Main,
}

#[derive(Deserialize, Debug)]
pub struct Weather {
    pub main: String,
    pub description: String,
}

#[derive(Deserialize, Debug)]
pub struct Main {
    pub temp: f64,
    pub feels_like: f64,
    pub temp_min: f64,
    pub temp_max: f64,
}

pub fn get_weather_data_esp<C: Connection>(client: &mut Client<C>, api_key: &str) -> Result<WeatherData, Box<dyn std::error::Error>> {
    let city = "Küsnacht,CH";
    let url = format!("http://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric", city, api_key);

    let mut response = client.get(&url)?.submit()?;

    let status = response.status();
    if status != 200 {
        return Err(format!("Failed to fetch weather data. Status: {}", status).into());
    }

    let mut body = Vec::new();
    response.read_to_end(&mut body)?;
    let weather_data: WeatherData = serde_json::from_slice(&body)?;

    Ok(weather_data)
}
