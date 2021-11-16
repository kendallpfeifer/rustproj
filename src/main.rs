use twilio::Client;
use twilio::OutboundMessage;

extern crate openweather;
use std::io::stdout;
use std::io::stdin;
use std::io::Write;

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

    // let loc = LocationSpecifier::CityAndCountryName{city:"College Park", country:"USA"};
    // let weather = openweather::get_current_weather(loc, API_KEY).unwrap();
    // println!("Right now in College Park, MD it is {}", weather.main.temp);
    let cityname = &city;
    let loc = LocationSpecifier::CityAndCountryName{city, country};
    let weather = openweather::get_current_weather(&loc, API_KEY, &Settings::default()).unwrap();
    //println!("Right now in {} it is {}F", cityname, weather.main.temp);


    let app_id = "ACd52baed8b7aa3964748b11a1c2476305";
    let auth_token = "64e0fde8442f7a0b0447899527e67b97";
    let client = Client::new(app_id, auth_token);
    let from = "17258884016";
    let body = format!("Right now it is {}F", weather.main.temp);
    let msg = OutboundMessage::new(from, &number, &body);
    client.send_message(msg).await; 
}