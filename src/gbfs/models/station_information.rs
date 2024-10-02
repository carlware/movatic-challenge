use serde::{Deserialize};

#[derive(Deserialize, Debug, Clone)]
pub struct RentalUris {
    pub ios: String,
    pub android: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct StationInformation {
    pub lon: f64,
    pub lat: f64,
    pub rental_uris: RentalUris,
    pub _bcycle_station_type: String,
    pub region_id: Option<String>,
    pub address: String,
    pub name: String,
    pub station_id: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct StationsInformation {
    pub stations: Vec<StationInformation>,
}

#[derive(Deserialize, Debug)]
pub struct StationInformationData {
    pub ttl: i64,
    pub data: StationsInformation,
    pub version: String,
    pub last_updated: i64,
}