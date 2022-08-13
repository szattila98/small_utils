use chrono::{NaiveDate, NaiveDateTime};
use structopt::StructOpt;

use crate::model::{DailyData, DailyUnits, HourlyData, HourlyUnits, WeatherData};

#[derive(Debug, StructOpt)]
#[structopt(name = "tempestas")]
/// A weather forecast CLI
pub enum Args {
    /// Weather summary of today
    Summary {
        /// Plus days to forecast in addition, max is 7
        #[structopt(short, long, conflicts_with_all = &["start-date","end-date"])]
        day_no: Option<u8>,
        /// Start date to show weather data for, max interval is 8 days, for a maximum of 7 days into the future and 2 months into the past, e.g. 2020-01-01
        #[structopt(short, long, requires = "end-date")]
        start_date: Option<NaiveDate>,
        /// End date to show weather data for, max interval is 8 days, for a maximum of 7 days into the future and 2 months into the past, e.g. 2020-01-01
        #[structopt(short, long, requires = "start-date")]
        end_date: Option<NaiveDate>,
    },
    /// Detailed weather forecast for today
    Detailed {
        /// Detailed weather forecast for a specific date
        #[structopt(short, long)]
        specific_date: Option<NaiveDate>,
    },
}

pub trait Forecast {
    fn print(&self);
}

impl Forecast for WeatherData<DailyUnits, DailyData> {
    fn print(&self) {
        let day_no = self.data.time.len();
        for i in 0..day_no {
            println!(
                "{}. ================ Day of {} ==================",
                i + 1,
                self.data.time[i]
            );
            println!(
                "The temperature will be between {}{} and {}{}",
                self.data.temperature_2m_min[i],
                self.units.temperature_2m_min,
                self.data.temperature_2m_max[i],
                self.units.temperature_2m_max
            );
            if self.data.precipitation_sum[i] == 0.0 && self.data.precipitation_hours[i] == 0.0 {
                println!("No precipitation expected");
            } else {
                println!(
                    "The precipitation will be {}{}s, for a sum of {} {}ours",
                    self.data.precipitation_sum[i],
                    self.units.precipitation_sum,
                    self.data.precipitation_hours[i],
                    self.units.precipitation_hours
                );
            }
            println!(
                "The maximum wind speed will be {}{}, at 10 meters",
                self.data.windspeed_10m_max[i], self.units.windspeed_10m_max
            );
            let sunrise = NaiveDateTime::parse_from_str(&self.data.sunrise[i], "%Y-%m-%dT%H:%M")
                .expect("could not parse sunrise time")
                .time();
            let sunset = NaiveDateTime::parse_from_str(&self.data.sunset[i], "%Y-%m-%dT%H:%M")
                .expect("could not parse sunset time")
                .time();
            println!("The sun will rise at {sunrise}, and will set at {sunset}");
        }
        println!("========================================================");
    }
}

impl Forecast for WeatherData<HourlyUnits, HourlyData> {
    fn print(&self) {
        todo!()
    }
}
