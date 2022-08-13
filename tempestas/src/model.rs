use chrono::{DateTime, NaiveDateTime};
use serde::{Deserialize, Deserializer};

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct WeatherData<U, D> {
    pub latitude: f64,
    pub longitude: f64,
    pub generationtime_ms: f64,
    pub utc_offset_seconds: i64,
    pub timezone: String,
    pub timezone_abbreviation: String,
    pub elevation: f64,
    #[serde(alias = "hourly_units")]
    #[serde(alias = "daily_units")]
    pub units: U,
    #[serde(alias = "hourly")]
    #[serde(alias = "daily")]
    pub data: D,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DailyUnits {
    pub time: String,
    pub temperature_2m_max: String,
    pub temperature_2m_min: String,
    pub rain_sum: String,
    pub windspeed_10m_max: String,
    pub precipitation_hours: String,
    pub snowfall_sum: String,
    pub showers_sum: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct HourlyUnits {
    pub time: String,
    pub temperature_2m: String,
    pub cloudcover: String,
    pub windspeed_10m: String,
    pub precipitation: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DailyData {
    pub time: Vec<String>,
    pub temperature_2m_max: Vec<f64>,
    pub temperature_2m_min: Vec<f64>,
    pub rain_sum: Vec<f64>,
    pub windspeed_10m_max: Vec<f64>,
    pub precipitation_hours: Vec<f64>,
    pub snowfall_sum: Vec<f64>,
    pub showers_sum: Vec<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]

pub struct HourlyData {
    pub time: Vec<NaiveDateTime>,
    pub temperature_2m: Vec<f64>,
    pub cloudcover: Vec<f64>,
    pub windspeed_10m: Vec<f64>,
    pub precipitation: Vec<f64>,
}
