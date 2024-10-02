use std::fmt;
use std::fmt::Error;
use futures::future::{err, ok};
use futures::TryFutureExt;
use reqwest::Client;
use reqwest::header::{ACCEPT, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use movatic::gbfs::GFSClient;

// #[derive(Serialize, Deserialize)]
// struct Feed {
//     pub url: String,
//     pub name: String,
// }
//
// #[derive(Serialize, Deserialize)]
// struct Data {
//     pub feeds: Vec<Feed>,
// }
//
// #[derive(Serialize, Deserialize)]
// struct GfsData {
//     pub en: Data,
// }
//
// #[derive(Serialize, Deserialize, Debug)]
// struct GfsFeed {
//     pub ttl: i64,
//     pub data: GfsData,
//     pub version: String,
//     pub last_updated: i64,
// }


// #[derive(Serialize, Deserialize, Debug)]
// struct NumBikesAvailableTypes {
//     pub electric: i64,
//     pub smart: i64,
//     pub classic: i64,
// }
//
// #[derive(Serialize, Deserialize, Debug)]
// struct Stations {
//     pub is_returning: i64,
//     pub is_renting: i64,
//     pub is_installed: i64,
//     pub num_docks_available: i64,
//     pub num_bikes_available: i64,
//     pub last_reported: i64,
//     pub num_bikes_available_types: NumBikesAvailableTypes,
//     pub station_id: String,
// }
//
// #[derive(Serialize, Deserialize, Debug)]
// struct GSFData {
//     pub stations: Vec<Stations>,
// }
//
// #[derive(Serialize, Deserialize, Debug)]
// struct GFSStatus {
//     pub ttl: i64,
//     pub last_updated: i64,
//     pub data: GSFData,
//     pub version: String,
// }
//
//
// #[derive(Serialize, Deserialize, Debug,Clone)]
// struct RentalUris {
//     pub ios: String,
//     pub android: String,
// }
//
// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct StationInformation {
//     pub lon: f64,
//     pub lat: f64,
//     pub rental_uris: RentalUris,
//     pub _bcycle_station_type: String,
//     pub region_id: Option<String>,
//     pub address: String,
//     pub name: String,
//     pub station_id: String,
// }
//
// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct Data {
//     pub stations: Vec<StationInformation>,
// }
//
// #[derive(Serialize, Deserialize, Debug)]
// struct GFSStatusInformation {
//     pub ttl: i64,
//     pub data: Data,
//     pub version: String,
//     pub last_updated: i64,
// }
//
// struct GFSClient {
//     http_client: Client,
//     base_url: String,
// }
//
// #[derive(Debug)]
// struct DoubleError;
//
// // Generation of an error is completely separate from how it is displayed.
// // There's no need to be concerned about cluttering complex logic with the display style.
// //
// // Note that we don't store any extra info about the errors. This means we can't state
// // which string failed to parse without modifying our types to carry that information.
// impl fmt::Display for DoubleError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "invalid first item to double")
//     }
// }
//
// impl GFSClient {
//     pub fn new(
//         base_url: String,
//         timeout: std::time::Duration,
//     ) -> Self {
//         let http_client = Client::builder().timeout(timeout).build().unwrap();
//         GFSClient{
//             http_client,
//             base_url,
//         }
//     }
//
//     pub fn search_by_id<'a>(
//         id: &String,
//         stations: &'a Vec<StationInformation>,
//     ) -> Option<&'a StationInformation> {
//         stations.iter().find(|x| x.station_id == *id)
//     }
//
//     pub async fn station_status(
//         &self,
//     ) -> Result<GFSStatus, reqwest::Error>  {
//         let url = format!("{}/station_status.json", self.base_url);
//
//         match self.http_client
//             .get(url)
//             .header(ACCEPT, "application/json")
//             .send()
//             .await?
//             .error_for_status() {
//                 Ok(response) => Ok(response.json::<GFSStatus>().await?),
//                 Err(e) => Err(e)
//         }
//     }
//
//     pub async fn station_information(
//         &self,
//     ) -> Result<GFSStatusInformation, reqwest::Error>  {
//         let url = format!("{}/station_information.json", self.base_url);
//         match self.http_client
//             .get(url)
//             .header(ACCEPT, "application/json")
//             .send()
//             .await?
//             .error_for_status() {
//                 Ok(response) => Ok(response.json::<GFSStatusInformation>().await?),
//                 Err(e) => Err(e)
//         }
//     }
//
//     pub async fn station_information_by_id<>(
//         self,
//         id: &String,
//     // ) {
//     ) -> Result<StationInformation, reqwest::Error>  {
//         match self.station_information().await {
//             Ok(res) => {
//                 let o = res.data.stations.iter().find(|x| x.station_id == *id).unwrap();
//                 return Ok(o.clone());
//             },
//                 // println!("station {:?}", obj);
//                 // match obj {
//                 //     Ok(o) => println!("{:?}",o),
//                 //     None => println!("not found")
//                 // }
//             // }
//             Err(e) => Err(e)
//         }
//     }
// }

async fn run1(c:  &GFSClient) {
    match c.get_station_status().await {
        Ok(res) => {

            println!("ok {:?}", res.data.stations[1]);
            match c.get_station_information().await {
                Ok(station_info) => {
                    let obj = GFSClient::search_by_id(&res.data.stations[1].station_id, &station_info.data.stations);
                    println!("obj main {:?}", obj)
                }
                Err(err) => {
                    println!("some err {:?}", err)
                }
            }
            // let obj = c.station_information_by_id(&res.data.stations[1].station_id).await;
            // println!("obj main {:?}", obj)
        }
        Err(err) => {
            println!("some err {:?}", err)
        }
    }
}

async fn run(client:  &GFSClient) -> Result<(), reqwest::Error>{
    let station_status = client.get_station_status().await?;
    let station_id = &station_status.data.stations[1].station_id;

    let status_info = client.get_station_information().await?;
    let obj = GFSClient::search_by_id(station_id, &status_info.data.stations);
    println!("obj main {:?}", obj);
    // println!("obj main\n {}", serde_json::to_string_pretty(&obj).unwrap());

    Ok(())
}

#[tokio::main]
async fn main() {
    // stations
    // station_information
    let client = GFSClient::new(
        String::from("https://gbfs.bcycle.com/bcycle_madison/"),
        std::time::Duration::from_millis(4000),
    );

    // let _ = run(&client).await.unwrap_or_else(|err| {
    //     eprintln!("Application error: {err}");
    // });

    if let Err(err) = run(&client).await {
        eprintln!("Application error: {err}");
    }
}
