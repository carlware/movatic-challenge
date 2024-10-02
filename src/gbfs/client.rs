use reqwest::Client;
use reqwest::header::ACCEPT;
use serde::de::DeserializeOwned;
use crate::gbfs::models::station_information::{StationInformation, StationInformationData};
use crate::gbfs::models::station_status::{StationStatusData};

const STATION_STATUS: &str = "station_status";
const STATION_INFORMATION: &str = "station_information";

pub struct GFSClient {
    http_client: Client,
    base_url: String,
}

impl GFSClient {
    pub fn new(
        base_url: String,
        timeout: std::time::Duration,
    ) -> Self {
        let http_client = Client::builder().timeout(timeout).build().unwrap();
        GFSClient{
            http_client,
            base_url,
        }
    }

    pub fn search_by_id<'a>(
        id: &String,
        stations: &'a Vec<StationInformation>,
    ) -> Option<&'a StationInformation> {
        stations.iter().find(|x| x.station_id == *id)
    }

    pub async fn get_station_status(
        &self,
    ) -> reqwest::Result<StationStatusData>  {
        self.get::<StationStatusData>(STATION_STATUS).await
    }

    pub async fn get_station_information(
        &self,
    ) -> reqwest::Result<StationInformationData>  {
        self.get::<StationInformationData>(STATION_INFORMATION).await
    }

    async fn get<T: DeserializeOwned>(&self, endpoint: &str) -> reqwest::Result<T> {
        self.http_client
            .get(format!("{}/{}.json", self.base_url, endpoint))
            .header(ACCEPT, "application/json")
            .send()
            .await?.
            json::<T>()
            .await
        // return Ok(response)
        // return match self.http_client
        //     .get(format!("{}/{}.json", self.base_url, endpoint))
        //     .header(ACCEPT, "application/json")
        //     .send()
        //     .await?
        //     .error_for_status() {
        //     Ok(response) => {
        //         // let res = response.json::<T>().await?;
        //         println!("res {:?}", response.status());
        //         Ok(response.json::<T>().await?)
        //     },
        //     Err(e) => {
        //         println!("err {} message {}", endpoint, e);
        //         Err(e)
        //     }
        // }
    }
}
