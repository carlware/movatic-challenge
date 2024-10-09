use serde_aux::field_attributes::{deserialize_number_from_string};
use crate::gbfs::GBFSClient;

#[derive(serde::Deserialize, Clone)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub gbfs: GBFSClientSettings,
}

#[derive(serde::Deserialize, Clone)]
pub struct ApplicationSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
}

#[derive(serde::Deserialize, Clone)]
pub struct GBFSClientSettings {
    pub url: String,
    #[serde(deserialize_with = "deserialize_number_from_string", default = "default_resource")]
    pub timeout_ms: u64,
}

fn default_resource() -> u64 {
    3000
}

impl GBFSClientSettings {
    pub fn client(self) -> GBFSClient {
        let timeout = self.timeout();
        GBFSClient::new(
            self.url,
            timeout,
        )
    }

    fn timeout(&self) -> std::time::Duration {
        std::time::Duration::from_millis(self.timeout_ms)
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .add_source(
            config::File::new("configuration.yaml", config::FileFormat::Yaml)
                .required(false)
        )
        .build()?;

    settings.try_deserialize::<Settings>()
}
