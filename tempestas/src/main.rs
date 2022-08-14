use crate::{
    cli::{Args, Forecast},
    location::get_geoip_data,
    model::{DailyData, DailyUnits, HourlyData, HourlyUnits, WeatherData},
};
use chrono::{Duration, Local};
use reqwest::blocking::get;
use std::ops::Add;
use structopt::StructOpt;

pub mod cli;
pub mod location;
pub mod model;

const HOURLY_VARS: &[&str] = &[
    "temperature_2m",
    "cloudcover",
    "windspeed_10m",
    "precipitation",
];
const DAILY_VARS: &[&str] = &[
    "temperature_2m_min",
    "temperature_2m_max",
    "precipitation_sum",
    "precipitation_hours",
    "windspeed_10m_max",
    "sunrise",
    "sunset",
];

fn main() {
    let args = Args::from_args();
    println!("Tempestas\n");
    println!("Locating you in the world by your ip address...");
    let ip_data = get_geoip_data();
    println!(
        "You are located in {}, {} in {}, according to your ip address",
        ip_data.city_name, ip_data.country_name, ip_data.continent_name
    );
    println!("This may be a little off, but shouldn't be too far off\n");
    println!("Getting weather data based on your location");
    let current_date = chrono::Local::today().naive_local();
    let forecast: Box<dyn Forecast> = match args {
        Args::Summary {
            day_no,
            start_date,
            end_date,
        } => {
            let (start_date, end_date) = if let Some(day_no) = day_no {
                let start_date = current_date;
                let end_date = current_date.add(Duration::days(day_no.into()));
                (start_date, end_date)
            } else if let Some(start_date) = start_date {
                (
                    start_date,
                    end_date.expect("end date should have been specified, and structopt should not let it not be - contact the nearest comissar to fix this"),
                )
            } else {
                (current_date, current_date)
            };

            if start_date > end_date {
                println!("Start date must be before the end date");
                return;
            }
            if end_date - start_date > Duration::days(7) {
                println!("The specified interval should not be more than 8 days");
                return;
            }
            if current_date > start_date && current_date - start_date > Duration::days(60) {
                println!(
                    "Only two months of data is available from the past, search for more recent dates"
                );
                return;
            }
            if current_date < start_date && end_date - current_date > Duration::days(7) {
                println!("Only 7 days of forecast is available not a day more, I am not an oracle");
                return;
            }
            if start_date == end_date {
                println!("for {}...", start_date);
            } else {
                println!("for the {} - {} interval...", start_date, end_date);
            }

            let url = format!(
                "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&timezone={}&start_date={}&end_date={}&daily={}", 
                ip_data.latitude, ip_data.longitude, ip_data.timezone, start_date.format("%Y-%m-%d"), end_date.format("%Y-%m-%d"), DAILY_VARS.join(",")
            );
            // println!("{}", url);
            let weather_data = get(url)
                .expect("could not get weather data")
                .json::<WeatherData<DailyUnits, DailyData>>()
                .expect("could not parse response");
            Box::new(weather_data)
        }

        Args::Detailed { specific_date } => {
            let specific_date = match specific_date {
                Some(date) => date,
                None => Local::now().naive_local().date(),
            };
            let formatted = specific_date.format("%Y-%m-%d");

            if current_date > specific_date && current_date - specific_date > Duration::days(60) {
                println!(
                    "Only two months of data is available from the past, search for more recent dates"
                );
                return;
            }
            if current_date < specific_date && specific_date - current_date > Duration::days(7) {
                println!("Only 7 days of forecast is available not a day more, I am not an oracle");
                return;
            }
            println!("for {}...", specific_date);

            let url =
                format!(
                    "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&start_date={}&end_date={}&hourly={}",
                    ip_data.latitude, ip_data.longitude, formatted, formatted, HOURLY_VARS.join(",")
                );
            // println!("{}", url);
            let weather_data = get(url)
                .expect("could not get weather data")
                .json::<WeatherData<HourlyUnits, HourlyData>>()
                .expect("could not parse response");
            Box::new(weather_data)
        }
    };
    forecast.print();
    println!("\nHave a nice day!")
}
