use twilio::Client;
use twilio::OutboundMessage;

extern crate openweather;
use std::io::stdout;
use std::io::stdin;
use std::io::Write;
use std::process::Command;

use openweather::LocationSpecifier;
use openweather::Settings;
static API_KEY: &str = "0ec1b697947476413186e8044c15a12f";

#[tokio::main]
async fn main() {
    use std::io::{stdin,stdout,Write};
    let mut number=String::new();
    print!("Please enter your phone number (with country code): ");
    let _=stdout().flush();
    stdin().read_line(&mut number).expect("Did not enter a correct string");
    if let Some('\n')=number.chars().next_back() {
        number.pop();
    }
    if let Some('\r')=number.chars().next_back() {
        number.pop();
    }
    let mut city=String::new();
    print!("Please enter your city: ");
    let _=stdout().flush();
    stdin().read_line(&mut city).expect("Did not enter a correct string");
    if let Some('\n')=city.chars().next_back() {
        city.pop();
    }
    if let Some('\r')=city.chars().next_back() {
        city.pop();
    }
    let mut country=String::new();
    print!("Please enter your country: ");
    let _=stdout().flush();
    stdin().read_line(&mut country).expect("Did not enter a correct string");
    if let Some('\n')=country.chars().next_back() {
        country.pop();
    }
    if let Some('\r')=country.chars().next_back() {
        country.pop();
    }

    let cityname = &city;
    let loc = LocationSpecifier::CityAndCountryName{city, country};
    let weather = openweather::get_current_weather(&loc, API_KEY, &Settings::default()).unwrap();

    let app_id = "ACd52baed8b7aa3964748b11a1c2476305";
    let auth_token = "64e0fde8442f7a0b0447899527e67b97";
    let client = Client::new(app_id, auth_token);
    let from = "17258884016";
    let tempfar = convert_to_f(weather.main.temp);
    let body = analyzeweather(weather);
    let mut child = Command::new("sleep").arg("100").spawn().unwrap();
    let _result = child.wait().unwrap();
    print!("{}\n", body);
    let msg = OutboundMessage::new(from, &number, &body);
    client.send_message(msg).await; 
}

fn analyzeweather(weather: openweather::WeatherReportCurrent) -> String {
    let min = convert_to_f(weather.main.temp_min);
    let max = convert_to_f(weather.main.temp_max);
    let mut report = format!("Today, there is a max of {:.0}F and a min of {:.0}F.", max, min);

    if weather.rain != None {
        report.push_str(" It is going to rain, so bring an umbrella!");
    } 
    if weather.wind.speed * 2.237 >= 10.0 {
        let temp = format!(" The wind will be {:.2} MPH today, so bring a coat!", weather.wind.speed * 2.237);
        report.push_str(&temp);
    } 
    if weather.clouds.all >= 60 {
        report.push_str(" It will be mostly cloudy.");
    }
    if weather.clouds.all < 60 {
        report.push_str(" It will be mostly sunny, bring sunglasses!");
    } 
    if weather.snow != None {
        report.push_str(" It is going to snow, so bundle up!");
    }
    return report;

}

fn convert_to_f(temp: f32) -> f32 {
    return (temp - 273.15) * (9.0/5.0) + 32.0;
}