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
    use claims::{assert_err, assert_ok};
    use wiremock::{Mock, MockServer, ResponseTemplate};
    use wiremock::matchers::any;
    use crate::gbfs::GBFSClient;
    use crate::gbfs::models::station_status::{StationsStatus, StationStatusData};

    fn gbfs_client(base_url: String) -> GBFSClient {
        GBFSClient::new(
            base_url,
            std::time::Duration::from_millis(200),
        )
    }

    #[tokio::test]
    async fn get_stations_status_if_api_returns_200() {
        let mock_server = MockServer::start().await;
        let gbfs_client = gbfs_client(mock_server.uri());

        let body = StationStatusData{
            ttl: 0,
            last_updated: 0,
            data: StationsStatus{
                stations: vec![],
            },
            version: "".to_string(),
        };

        Mock::given(any())
            .respond_with(ResponseTemplate::new(200).set_body_json(body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let response = gbfs_client
            .get_station_status()
            .await;

        assert_ok!(response);
    }

    #[tokio::test]
    async fn get_error_if_api_returns_500() {
        let mock_server = MockServer::start().await;
        let gbfs_client = gbfs_client(mock_server.uri());


        Mock::given(any())
            .respond_with(ResponseTemplate::new(500).set_body_string("server error"))
            .expect(1)
            .mount(&mock_server)
            .await;

        let response = gbfs_client
            .get_station_status()
            .await;

        assert_err!(response);
    }
}