use actix_web::{Responder, web};
use crate::gbfs::GBFSClient;


pub async fn station_information(
    gbfs_client: web::Data<GBFSClient>
) -> Result<impl Responder, actix_web::Error> {
    let station_info = gbfs_client.get_station_information()
        .await
        .map_err(|e|actix_web::error::ErrorInternalServerError(e))?;

    Ok(web::Json(station_info))
}