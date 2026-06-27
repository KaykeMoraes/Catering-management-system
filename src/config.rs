use std::env;

pub struct EnvironmentVariables {
    address: String,
    port: u16,
    database_url: String,
    allowed_origin: String,
}

impl EnvironmentVariables {
    pub fn load() -> Result<Self, String> {
        dotenvy::dotenv().ok();

        let address =
            env::var("ADDRESS").map_err(|e| format!("Failed to load ADDRESS variable: {}", e))?;

        let port = env::var("PORT")
            .map_err(|err| format!("Failed to load PORT variable: {}", err))?
            .parse()
            .map_err(|err| format!("Failed to parse string to u16: {}", err))?;

        let database_url = env::var("DATABASE_URL")
            .map_err(|e| format!("Failed to load DATABASE_URL variable: {}", e))?;

        let allowed_origin = env::var("ALLOWED_ORIGIN")
            .map_err(|e| format!("Failed to load ALLOWED_ORIGIN variable: {}", e))?;

        Ok(Self {
            address,
            port,
            database_url,
            allowed_origin,
        })
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

    pub fn allowed_origin(&self) -> &str {
        &self.allowed_origin
    }
}
