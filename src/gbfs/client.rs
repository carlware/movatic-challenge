use reqwest::Client;
use reqwest;
use reqwest::header::ACCEPT;
use serde::de::{DeserializeOwned};
use crate::gbfs::models::station_information::{StationInformationData};
use crate::gbfs::models::station_status::{StationStatusData};

const STATION_STATUS: &str = "station_status";
const STATION_INFORMATION: &str = "station_information";

pub struct GBFSClient {
    http_client: Client,
    base_url: String,
}

impl GBFSClient {
    pub fn new(
        base_url: String,
        timeout: std::time::Duration,
    ) -> Self {
        let http_client = Client::builder().timeout(timeout).build().unwrap();
        GBFSClient {
            http_client,
            base_url,
        }
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
            .await?
            .json::<T>()
            .await
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize};
    use serde::de::{DeserializeOwned};
    use reqwest;
    use serde_json;

    // This `derive` requires the `serde` dependency.
    #[derive(Deserialize, Debug)]
    pub struct Ip {
        origin: u64,
    }

    async fn get<T: DeserializeOwned>(endpoint: &str) -> reqwest::Result<T> {
        reqwest::get("http://httpbin.org/ip")
            .await?
            .json::<T>()
            .await
    }

    #[tokio::test]
    async fn send_email_sends_the_expected_request() -> Result<(), reqwest::Error>{
        let ip = get::<Ip>("http://httpbin.org/ip").await;

        println!("ip: {:?}", ip);
        Ok(())
    }
}