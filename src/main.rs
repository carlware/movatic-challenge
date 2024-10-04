use std::net::TcpListener;
use movatic::configuration::get_configuration;
use movatic::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration()
        .expect("Failed to read configuration.");

    let gbfs_client = configuration.gbfs_client.client();
    let address = format!("{}:{}",
                          configuration.application.host,
                          configuration.application.port,
    );

    let listener = TcpListener::bind(address)?;
    run(listener, gbfs_client)?.await
}
