use reqwest::blocking::get;
use serde::Deserialize;

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct IpData {
    pub ip_address: String,
    pub latitude: f64,
    pub longitude: f64,
    pub postal_code: String,
    pub continent_code: String,
    pub continent_name: String,
    pub country_code: String,
    pub country_name: String,
    pub region_code: String,
    pub region_name: String,
    pub province_code: String,
    pub province_name: String,
    pub city_name: String,
    pub timezone: String,
}

pub fn get_geoip_data() -> IpData {
    get("https://api.geoip.rs/")
        .expect("could make a request to the geoip api")
        .json::<IpData>()
        .expect("could not parse response")
}
