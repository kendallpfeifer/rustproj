use twilio::Client;
use twilio::OutboundMessage;

extern crate openweather;
// use std::io::stdout;
// use std::io::stdin;
// use std::io::Write;
use std::process::Command;

use openweather::LocationSpecifier;
use openweather::Settings;
static API_KEY: &str = "0ec1b697947476413186e8044c15a12f";

extern crate rand;
use rand::Rng;

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

    //let cityname = &city;
    let loc = LocationSpecifier::CityAndCountryName{city, country};
    let weather = openweather::get_current_weather(&loc, API_KEY, &Settings::default()).unwrap();

    let app_id = "ACd52baed8b7aa3964748b11a1c2476305";
    let auth_token = "64e0fde8442f7a0b0447899527e67b97";
    let client = Client::new(app_id, auth_token);
    let from = "17258884016";
    //let tempfar = convert_to_f(weather.main.temp);
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

    // Generate the basic report with temperature values
    let mut report = format!("Today, there is a max of {:.0}F and a min of {:.0}F.\n", max, min);

    // Add randomized report based on rain status and amounts
    report.push_str(&gen_rain_report(&weather));

    // Add randomized report based on wind amount
    report.push_str(&gen_wind_report(&weather));


    if weather.clouds.all >= 60 {
        report.push_str("It will be mostly cloudy.");
    }
    if weather.clouds.all < 60 {
        report.push_str("It will be mostly sunny, bring sunglasses!");
    } 
    if weather.snow != None {
        report.push_str("It is going to snow, so bundle up!");
    }
    return report;

}


fn gen_rain_report (weather: &openweather::WeatherReportCurrent) -> String {
    let mut rng = rand::thread_rng();
    if weather.rain != None {
        let amt = weather.rain.as_ref().unwrap().three_h.unwrap() / 3.0;
        let rain_str = {
            if amt < 2.5 {
                match rng.gen_range(0..3) {
                    0 => &format!("It's barely even raining, only {:.0}mm per hour so far\n", amt),
                    1 => "Only light rain today! Probably won't need an umbrella\n",
                    _ => "We have what the experts call a 'Light Drizzle'\n",
                }
            } else if amt < 7.6 {
                match rng.gen_range(0..3) {
                    0 => &format!("Moderate rain today at {:.0}mm per hour\n", amt),
                    1 => "The rain is picking up a bit, better bring an umbrella\n",
                    _ => "Make sure to pack a jacket and an umbrella today because the rain is no joke\n"
                }
            } else {
                match rng.gen_range(0..3) {
                    0 => &format!("Whoa! Crazy heavy rain today clocking in at {:.0}mm per hour!\n", amt),
                    1 => "Heavy rainfall today! Make sure to close your windows so you dont end up with an indoor swimming pool\n",
                    2 => "So much rain today! If it weren't for the puddle jumping potential, I'd recommend staying inside\n",
                    _ => "Heavy rain today! or as some call it: 'a real toad-strangler'\n"
                }
            }
        };
        String::from(rain_str)
    } else {
        let no_rain = {
            match rng.gen_range(0..5) {
                0 => "No rain today!\n",
                1 => "No need for rain boots.\n",
                2 => "Darn, no rain puddles to jump in today\n",
                _ => "",
            }
        };
        String::from(no_rain)
    }
}

fn gen_wind_report(weather:&openweather::WeatherReportCurrent) -> String {
    let wind_val = weather.wind.speed * 2.237;
    let mut rng = rand::thread_rng();
    let wind_str = {
        if wind_val < (9.0/1.15) {
            // Light Breeze
            match rng.gen_range(0..5) {
                0 => "Almost no wind today",
                1 => "There is currently a light breeze",
                _ => ""
            }
        } else if wind_val < (24.0/1.15) {
            // Strong  breeze
            match rng.gen_range(0..3) {
                0 => &format!("The wind is picking up a bit at {:.0}mph", wind_val),
                1 => "A bit of a stronger breeze coming your way, expect a cool down",
                _ => "It'll feel cooler than usual with the breeze today",
            }
        } else if wind_val < (44.0/1.15) {
            // Gale/Storm winds
            match rng.gen_range(0..3) {
                0 => "Hold onto your hat!! Today's wind is strong!",
                1 => &format!("The wind today clocks in at {:.0}mph! Almost gale force", wind_val),
                _ => "Strong winds today! Be careful and pack a jacket!!",
            }
        } else {
            // Hurricane!!
            match rng.gen_range(0..2) {
                0 => "Take shelter! You do not want to be outside in these winds",
                _ => "Storm force winds are coming your way! Stay safe out there",
            }
        }
    };
    let mut wind_string = String::from(wind_str);
    wind_string.push('\n');
    wind_string
}


fn gen_cloud_report(weather: &openweather::WeatherReportCurrent) -> String {
    let rng = rand::thread_rng();
    String::default()
}

fn gen_humidity_report() {

}

fn convert_to_f(temp: f32) -> f32 {
    return (temp - 273.15) * (9.0/5.0) + 32.0;
}