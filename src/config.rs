use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub database_url: String,
    pub port: Option<u16>,
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
