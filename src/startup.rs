use actix_web::{web, App, HttpServer};
use actix_web::dev::Server;
use std::net::TcpListener;
use actix_web::middleware::Logger;
use env_logger::Env;
use crate::gbfs;
use crate::routes::{health_check, station_information, station_status};
use actix_cors::Cors;

pub fn run(
    listener: TcpListener,
    gbfs_client: gbfs::GBFSClient,
) -> Result<Server, std::io::Error> {

    let gbfs = web::Data::new(gbfs_client);

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let server = HttpServer::new(move || {
        let cors = Cors::default().allow_any_origin().send_wildcard();

        App::new()
            .route("health_check", web::get().to(health_check))
            .route("station_status", web::get().to(station_status))
            .route("station_information", web::get().to(station_information))
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(gbfs.clone())
    })
        .listen(listener)?
        .run();

    Ok(server)
}
