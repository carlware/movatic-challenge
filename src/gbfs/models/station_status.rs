use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize,  Debug)]
pub struct BikesAvailable {
    pub electric: i64,
    pub smart: i64,
    pub classic: i64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct StationStatus {
    pub is_returning: i64,
    pub is_renting: i64,
    pub is_installed: i64,
    pub num_docks_available: i64,
    pub num_bikes_available: i64,
    pub last_reported: i64,
    pub num_bikes_available_types: BikesAvailable,
    pub station_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct StationsStatus {
    pub stations: Vec<StationStatus>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct StationStatusData {
    pub ttl: i64,
    pub last_updated: i64,
    pub data: StationsStatus,
    pub version: String,
}
