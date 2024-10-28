/*
*   Made with ❤ by Faisal Rafiq. All Right Reserved!
*/
use std::io;
use serde::Deserialize;
use colored::*;

#[derive(Deserialize, Debug)]
struct WeatherResponse {
    weather: Option<Vec<Weather>>, // Make this Option to handle missing fields
    main: Option<Main>, // Make this Option to handle missing fields
    wind: Option<Wind>, // Make this Option to handle missing fields
    name: Option<String>, // Make this Option to handle missing fields
    cod: Option<u16>, // Status code to check for errors
    message: Option<String>, // Error message if present
}

#[derive(Deserialize, Debug)]
struct Weather {
    description: String,
}

#[derive(Deserialize, Debug)]
struct Main {
    temp: f64,
    humidity: f64,
    pressure: f64,
}

#[derive(Deserialize, Debug)]
struct Wind {
    speed: f64,
}

fn get_weather_info(city: &str, country_code: &str, api_key: &str) -> Result<WeatherResponse, reqwest::Error> {
    let url = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid={}",
        city, country_code, api_key
    );

    let response = reqwest::blocking::get(&url)?;
    let response_json = response.json::<WeatherResponse>()?;
    Ok(response_json)
}

fn display_weather_info(response: &WeatherResponse) {
    if let Some(cod) = response.cod {
        if cod != 200 {
            if let Some(message) = &response.message {
                eprintln!("Error: {}", message.bright_red());
            } else {
                eprintln!("Error: Received status code {}", cod);
            }
            return;
        }
    }

    let description = response.weather.as_ref().map_or("N/A", |w| &w[0].description);
    let temperature = response.main.as_ref().map_or(0.0, |m| m.temp);
    let humidity = response.main.as_ref().map_or(0.0, |m| m.humidity);
    let pressure = response.main.as_ref().map_or(0.0, |m| m.pressure);
    let wind_speed = response.wind.as_ref().map_or(0.0, |w| w.speed);

    let weather_text = format!(
        "Weather in {}: {} {}
        > Temperature: {:.1}°C, 
        > Humidity: {:.1}%, 
        > Pressure: {:.1} hPa, 
        > Wind Speed: {:.1} m/s",
        response.name.as_ref().unwrap_or(&"Unknown".to_string()),
        description,
        get_temperature_emoji(temperature),
        temperature,
        humidity,
        pressure,
        wind_speed,
    );

    let weather_text_colored = match description {
        "clear sky" => weather_text.bright_yellow(),
        "few clouds" | "scattered clouds" | "broken clouds" => weather_text.bright_blue(),
        "overcast clouds" | "mist" | "haze" | "smoke" | "sand" | "dust" | "fog" | "squalls" => weather_text.dimmed(),
        "shower rain" | "rain" | "thunderstorm" | "snow" => weather_text.bright_cyan(),
        _ => weather_text.normal(),
    };

    println!("{}", weather_text_colored);
}

fn get_temperature_emoji(temperature: f64) -> &'static str {
    if temperature < 0.0 {
        "❄️"
    } else if temperature >= 0.0 && temperature < 10.0 {
        "☁️"
    } else if temperature >= 10.0 && temperature < 20.0 {
        "⛅"
    } else if temperature >= 20.0 && temperature < 30.0 {
        "🌤️"
    } else {
        "🔥"
    }
}

fn main() {
    println!("{}", "Welcome to Weather Station!".bright_yellow());

    loop {
        println!("{}", "Please enter the name of the city:".bright_green());

        let mut city = String::new();
        io::stdin().read_line(&mut city).expect("Failed to read input");
        let city = city.trim();

        println!("{}", "Please enter the country code (e.g., PK for Pakistan):".bright_green());

        let mut country_code = String::new();
        io::stdin().read_line(&mut country_code).expect("Failed to read input");
        let country_code = country_code.trim();

        let api_key = "YOUR_API_KEY"; //Add your API key here.

        match get_weather_info(&city, &country_code, api_key) {
            Ok(response) => {
                display_weather_info(&response);
            }
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        }

        println!("{}", "Do you want to search for weather in another city? (yes/no):".bright_green());
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let input = input.trim().to_lowercase();

        if input != "yes" {
            println!("Thank you for using our software!");
            break;
        }
    }
}
