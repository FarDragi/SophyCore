use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub database: DatabaseConfig,
}

#[derive(Deserialize, Debug)]
pub struct DatabaseConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub user: Option<String>,
    pub password: String,
    pub database: Option<String>,
}

impl Config {
    pub fn new() -> Self {
        info!("Loading config");
        Config::figment().extract().expect("Fail get config")
    }

    pub fn figment() -> Figment {
        Figment::new()
            .merge(Env::prefixed("APP_"))
            .merge(Toml::file("App.toml"))
    }
}
