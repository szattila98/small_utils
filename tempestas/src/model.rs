use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WeatherData<U, D> {
    #[serde(alias = "hourly_units")]
    #[serde(alias = "daily_units")]
    pub units: U,
    #[serde(alias = "hourly")]
    #[serde(alias = "daily")]
    pub data: D,
}

#[derive(Debug, Deserialize)]
pub struct DailyUnits {
    pub time: String,
    pub temperature_2m_min: String,
    pub temperature_2m_max: String,
    pub precipitation_sum: String,
    pub precipitation_hours: String,
    pub windspeed_10m_max: String,
    pub sunrise: String,
    pub sunset: String,
}

#[derive(Debug, Deserialize)]
pub struct DailyData {
    pub time: Vec<String>,
    pub temperature_2m_min: Vec<f64>,
    pub temperature_2m_max: Vec<f64>,
    pub precipitation_sum: Vec<f64>,
    pub precipitation_hours: Vec<f64>,
    pub windspeed_10m_max: Vec<f64>,
    pub sunrise: Vec<String>,
    pub sunset: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct HourlyUnits {
    pub time: String,
    pub temperature_2m: String,
    pub cloudcover: String,
    pub windspeed_10m: String,
    pub precipitation: String,
}

#[derive(Debug, Deserialize)]

pub struct HourlyData {
    pub time: Vec<String>,
    pub temperature_2m: Vec<f64>,
    pub cloudcover: Vec<f64>,
    pub windspeed_10m: Vec<f64>,
    pub precipitation: Vec<f64>,
}
