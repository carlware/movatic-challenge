use actix_web::{Responder, web};
use crate::gbfs::GBFSClient;

pub async fn station_status(
    gbfs_client: web::Data<GBFSClient>
) -> Result<impl Responder, actix_web::Error> {
    let station_date = gbfs_client.get_station_status()
        .await
        .map_err(|e|actix_web::error::ErrorInternalServerError(e))?;

    Ok(web::Json(station_date))
}
