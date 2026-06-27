use std::env;

use rocket::figment::Figment;

pub struct Config {
    address: String,
    port: u16,
}

impl Config {
    pub fn init() -> Self {
        let env_file = dotenvy::dotenv();
        if env_file.is_err() {
            error!("Error trying to load .env file: {}", env_file.unwrap_err());
            warn!("Failed to get .env file, relying on environment variables");
        }

        let port = env::var("PORT")
            .inspect_err(|err| error!("Error trying to load PORT variable: {}", err))
            .expect("Failed to get PORT variable");

        let port_str: u16 = port.parse().expect("Failed to parse PORT to u16");

        let address = env::var("ADRESS")
            .inspect_err(|err| error!("Error trying to load ADDRESS variable: {}", err))
            .expect("Failed to load ADDRESS variable");

        Self {
            address: address,
            port: port_str,
        }
    }
    pub fn to_figment(&self) -> Figment {
        Figment::new()
            .merge(("port", self.port))
            .merge(("address", &self.address))
    }
}
