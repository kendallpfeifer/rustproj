use twilio::Client;
use twilio::OutboundMessage;

use std::process::Command;

extern crate openweather;
use openweather::LocationSpecifier;
use openweather::Settings;
static API_KEY: &str = "0ec1b697947476413186e8044c15a12f";

extern crate rand;
use rand::Rng;

extern crate regex;

#[tokio::main]
async fn main() {
    use std::io::{stdin,stdout,Write};

    // Read the user's phone number from the command line
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

    // Read the user's city from the command line
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

    // Read the user's country from the command line
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

    print!("Great thanks! We will generate that report for you whenever you need it :)");
    
    // Get the desired alert time from the user
    let mut alert_time=String::new();
    print!("What time would you like to be reminded: ");
    let _=stdout().flush();
    stdin().read_line(&mut alert_time).expect("Did not enter a correct string");
    if let Some('\n')=alert_time.chars().next_back() {
        country.pop();
    }
    if let Some('\r')=alert_time.chars().next_back() {
        country.pop();
    }

    //let cityname = &city;
    let loc = LocationSpecifier::CityAndCountryName{city, country};
    let weather = openweather::get_current_weather(&loc, API_KEY, &Settings::default()).unwrap();

    let app_id = "ACd52baed8b7aa3964748b11a1c2476305";
    let auth_token = "64e0fde8442f7a0b0447899527e67b97";
    let client = Client::new(app_id, auth_token);
    let from = "17258884016";
    let body = analyzeweather(weather);
    print!("{}\n", body);
    //let mut child = Command::new("sleep").arg("100").spawn().unwrap();
    //let _result = child.wait().unwrap();
    
    //let msg = OutboundMessage::new(from, &number, &body);
    //client.send_message(msg).await; 
}


fn analyzeweather(weather: openweather::WeatherReportCurrent) -> String {
    let min = convert_to_f(weather.main.temp_min);
    let max = convert_to_f(weather.main.temp_max);

    // Generate the basic report with temperature values
    let mut report = format!("Heres your Weather Report for the day:\nToday, there is a max of {:.0}F and a min of {:.0}F.\n", max, min);

    // Add randomized report based on rain status and amounts
    report.push_str(&gen_rain_report(&weather));

    // Add randomized report based on snow status and amounts
    report.push_str(&gen_snow_report(&weather));

    // Add randomized report based on wind speeds
    report.push_str(&gen_wind_report(&weather));

    // Add randomized report based on cloud percentage
    report.push_str(&gen_cloud_report(&weather));


    return report;

}


fn gen_rain_report (weather: &openweather::WeatherReportCurrent) -> String {
    let mut rng = rand::thread_rng();
    if weather.rain != None {
        let amt = weather.rain.as_ref().unwrap().three_h.unwrap() / 3.0;
        let rain_str = {
            if amt < 2.5 {
                match rng.gen_range(0..3) {
                    0 => format!("It's barely even raining, only {:.0}mm per hour so far\n", amt),
                    1 => String::from("Only light rain today! Probably won't need an umbrella\n"),
                    _ => String::from("We have what the experts call a 'Light Drizzle'\n"),
                }
            } else if amt < 7.6 {
                match rng.gen_range(0..3) {
                    0 => format!("Moderate rain today at {:.0}mm per hour\n", amt),
                    1 => String::from("The rain is picking up a bit, better bring an umbrella\n"),
                    _ => String::from("Make sure to pack a jacket and an umbrella today because the rain is no joke\n"),
                }
            } else {
                match rng.gen_range(0..3) {
                    0 => format!("Whoa! Crazy heavy rain today clocking in at {:.0}mm per hour!\n", amt),
                    1 => String::from("Heavy rainfall today! Make sure to close your windows so you dont end up with an indoor swimming pool\n"),
                    2 => String::from("So much rain today! If it weren't for the puddle jumping potential, I'd recommend staying inside\n"),
                    _ => String::from("Heavy rain today! or as some call it: 'a real toad-strangler'\n"),
                }
            }
        };
        rain_str
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
            match rng.gen_range(0..3) {
                0 => String::from("Almost no wind today\n"),
                1 => String::from("There is currently a light breeze\n"),
                _ => String::from("")
            }
        } else if wind_val < (24.0/1.15) {
            // Strong  breeze
            match rng.gen_range(0..3) {
                0 => format!("The wind is picking up a bit at {:.0}mph\n", wind_val),
                1 => String::from("A bit of a stronger breeze coming your way, expect a cool down\n"),
                _ => String::from("It'll feel cooler than usual with the breeze today\n"),
            }
        } else if wind_val < (44.0/1.15) {
            // Gale/Storm winds
            match rng.gen_range(0..3) {
                0 => String::from("Hold onto your hat!! Today's wind is strong!\n"),
                1 => format!("The wind today clocks in at {:.0}mph! Almost gale force\n", wind_val),
                _ => String::from("Strong winds today! Be careful and pack a jacket!!\n"),
            }
        } else {
            // Hurricane!!
            match rng.gen_range(0..2) {
                0 => String::from("Take shelter! You do not want to be outside in these winds\n"),
                _ => String::from("Storm force winds are coming your way! Stay safe out there\n"),
            }
        }
    };
    
    wind_str
}


fn gen_cloud_report(weather: &openweather::WeatherReportCurrent) -> String {
    let cloud_val = weather.clouds.all;
    let mut rng = rand::thread_rng();

    let cloud_str = {
        if cloud_val <= 25 {
            // Sunny Skies!
            match rng.gen_range(0..2) {
                0 => "Skies are looking sunny, maybe pack some shades\n",
                1 => "Not a cloud in sight, enjoy the blue skies\n",
                _ => "",
            }
        } else if cloud_val <= 50 {
            match rng.gen_range(0..2) {
                0 => "Just a few clouds in the sky today\n",
                1 => "Great day for some cloud watching, not too many not too little\n",
                _ => "",
            }
        } else if cloud_val <= 75 {
            match rng.gen_range(0..2) {
                0 => "Pretty cloudy out there today, time to ditch the shades\n",
                1 => "You won't be needing any visors with clouds like these today\n",
                _ => "",
            }
        } else {
            match rng.gen_range(0..2) {
                0 => "It'll be a dark one out there today with all this cloud cover\n",
                1 => "Might want to pack a flashlight, these clouds mean it's gonna be dark\n",
                _ => "Seems like a storm's brewing with such high cloud cover!\n",
            }
        }
    };

    String::from(cloud_str)
}

// fn gen_humidity_report() {

// }

fn gen_snow_report(weather: &openweather::WeatherReportCurrent) -> String {
    let mut rng = rand::thread_rng();
    if weather.snow != None {
        let amt = weather.snow.as_ref().unwrap().three_h.unwrap() / 3.0;
        let snow_str = {
            if amt < 13.0 {
                match rng.gen_range(0..3) {
                    0 => format!("It's barely even snowing, only {:.0}mm per hour so far\n", amt),
                    1 => String::from("This snow wouldn't even be called a dusting right now\n"),
                    _ => String::from("The snow is hardly falling'\n"),
                }
            } else if amt < 25.0 {
                match rng.gen_range(0..3) {
                    0 => format!("Moderate snow today at {:.0}mm (just under an inch) per hour\n", amt),
                    1 => String::from("The snow is picking up a bit, better bring a nice coat\n"),
                    _ => String::from("Do you have your hat and scarf handy? because the snow today is picking up\n"),
                }
            } else if amt < 50.0 {
                match rng.gen_range(0..3) {
                    0 => format!("Wow lots of fresh snow today at {:.0}mm (almost 2 inches) per hour!!\n", amt),
                    1 => String::from("You do not want to be caught in this heavy snow without a hat and gloves!\n"),
                    _ => String::from("Make sure to pack a jacket and an wear boots today because the heavy snow is no joke\n"),
                }
             } else {
                match rng.gen_range(0..3) {
                    0 => format!("Whoa! Crazy heavy snow today clocking in at {:.0}mm per hour!\n", amt),
                    1 => String::from("This snow is almost a blizzard! I'd stay home if I were you\n"),
                    2 => String::from("Be safe out there today, heavy snow is coming your way!!\n"),
                    _ => String::from("This snow is looking good for a snow day! Fingers crossed\n"),
                }
            }
        };
        snow_str
    } else {
        let no_snow = {
            if convert_to_f(weather.main.temp_min) <= 40.0 {
                match rng.gen_range(0..3) {
                    0 => "No snow day today, sorry!\n",
                    1 => "No need for snow boots.\n",
                    _ => "",
                }
            } else {
                ""
            }
        };
        String::from(no_snow)
    }
}

fn convert_to_f(temp: f32) -> f32 {
    return (temp - 273.15) * (9.0/5.0) + 32.0;
}