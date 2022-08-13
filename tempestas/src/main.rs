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
    println!("This may be a little off, but should'nt be too far off\n");
    println!("Getting weather data based on this...");
    let forecast: Box<dyn Forecast> = match args {
        Args::Summary {
            day_no,
            start_date,
            end_date,
        } => {
            let (start_date, end_date) = if let Some(day_no) = day_no {
                let start_date = chrono::Local::today().format("%Y-%m-%d");
                let end_date = chrono::Local::today()
                    .add(Duration::days(day_no.into()))
                    .format("%Y-%m-%d");
                (start_date, end_date)
            } else if let Some(start_date) = start_date {
                (
                    start_date.format("%Y-%m-%d"),
                    end_date.expect("end date should have been specified, and structopt should not let it not be - contact the nearest comissar to fix this").format("%Y-%m-%d"),
                )
            } else {
                (
                    chrono::Local::today().format("%Y-%m-%d"),
                    chrono::Local::today().format("%Y-%m-%d"),
                )
            };
            let url = format!(
                "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&timezone={}&start_date={}&end_date={}&daily={}", 
                ip_data.latitude, ip_data.longitude, ip_data.timezone, start_date, end_date, DAILY_VARS.join(",")
            );
            let weather_data = get(url)
                .expect("could not get weather data")
                .json::<WeatherData<DailyUnits, DailyData>>()
                .expect("could not parse response");
            Box::new(weather_data)
        }
        Args::Detailed { specific_date } => {
            let date = match specific_date {
                Some(date) => date,
                None => Local::now().naive_local().date(),
            };
            let formatted = date.format("%Y-%m-%d");
            let url =
                format!(
                    "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&start_date={}&end_date={}&hourly={}",
                    ip_data.latitude, ip_data.longitude, formatted, formatted, HOURLY_VARS.join(",")
                );
            let weather_data = get(url)
                .expect("could not get weather data")
                .json::<WeatherData<HourlyUnits, HourlyData>>()
                .expect("could not parse response");
            Box::new(weather_data)
        }
    };
    forecast.print();
}
