use crate::{
    cli::Args,
    location::get_geoip_data,
    model::{HourlyData, HourlyUnits, WeatherData},
};
use chrono::Local;
use reqwest::blocking::get;
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
    let ip_data = get_geoip_data();
    match args {
        Args::Summary {
            day_no,
            start_date,
            end_date,
        } => todo!(),
        Args::Detailed { specific_date } => {
            let date = match specific_date {
                Some(date) => date,
                None => Local::now().naive_local().date(),
            };
            let formatted = date.format("%Y-%m-%d");
            let url =
                format!(
                    "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&hourly={}&start_date={}&end_date={}",
                    ip_data.latitude, ip_data.longitude, HOURLY_VARS.join(","), formatted, formatted
                );
            let weather_data = get(url)
                .expect("could not get weather data")
                .json::<WeatherData<HourlyUnits, HourlyData>>()
                .expect("could not parse response");
            println!("{:#?}", weather_data);
        }
    }

    // let start_date = chrono::Local::today().format("%Y-%m-%d");
    // let end_date = chrono::Local::today()
    //     .add(Duration::days(2))
    //     .format("%Y-%m-%d");
    // let url = format!("https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&timezone={}&start_date={}&end_date={}&daily=temperature_2m_max,temperature_2m_min,rain_sum,windspeed_10m_max,precipitation_hours,snowfall_sum,showers_sum", ip_data.latitude, ip_data.longitude, ip_data.timezone, start_date, end_date);
    // let weather_data = get(url)
    //     .expect("could not get weather data")
    //     .json::<WeatherData>()
    //     .expect("could not parse response");

    // println!("{args:?}");
    // println!("body = {:?}", weather_data);
    // println!("{}", chrono::Local::now());
}
