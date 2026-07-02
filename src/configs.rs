pub mod cors;

use std::env::var;

#[derive(Debug, Default)]
pub struct EnvironmentVariables {
    address: String,
    port: u16,
    database_url: String,
    cors_allowed_origin: String,
}

impl EnvironmentVariables {
    pub fn load() -> Self {
        info!("Trying to load environment variables.");

        let _env = match dotenvy::dotenv() {
            Ok(_) => {
                info!("Success loading .env file")
            }
            Err(err) => {
                error!("Error loading .env file: {err}");
                warn!("Error loading .env file, relying on environment variables")
            }
        };

        let mut environment_vars = EnvironmentVariables::default();

        info!("Trying to load PORT variable");
        match var("PORT") {
            Ok(port) => match port.parse::<u16>() {
                Ok(port) => {
                    info!("Loaded PORT variable with success");
                    environment_vars.port = port;
                }
                Err(err) => {
                    panic!("Failed to convert string to u16: {err}");
                }
            },
            Err(err) => {
                panic!("Failed to load PORT variable: {err}")
            }
        }

        info!("Trying to load ADDRESS variable");
        match var("ADDRESS") {
            Ok(address) => {
                info!("Loaded ADDRESS variable with success");
                environment_vars.address = address;
            }
            Err(err) => {
                panic!("Failed to load ADDRESS variable: {err}")
            }
        }

        info!("Trying to load DATABASE_URL variable");
        match var("DATABASE_URL") {
            Ok(database_url) => {
                info!("Loaded DATABASE_URL variable with success");
                environment_vars.database_url = database_url;
            }
            Err(err) => {
                panic!("Failed to load DATABASE_URL variable: {err}")
            }
        }

        info!("Trying to load ALLOWED_ORIGIN variable");
        match var("ALLOWED_ORIGIN") {
            Ok(allowed_origin) => {
                info!("Loaded ALLOWED_ORIGIN variable with success");
                environment_vars.cors_allowed_origin = allowed_origin;
            }
            Err(err) => {
                panic!("Failed to load ALLOWED_ORIGIN variable: {err}")
            }
        }
        environment_vars
    }

    pub fn address(&self) -> &str {
        &self.address
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn database_url(&self) -> &str {
        &self.database_url
    }

    pub fn cors_allowed_origin(&self) -> &str {
        &self.cors_allowed_origin
    }
}
