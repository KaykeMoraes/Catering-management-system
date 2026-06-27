use rocket::Config;

use crate::{config::EnvironmentVariables, repository::DatabaseConnection};

mod config;
mod repository;

#[macro_use]
extern crate log;

#[rocket::launch]
async fn launch() -> _ {
    env_logger::init();

    let env_vars = EnvironmentVariables::load().expect("Failed to load environment variables");
    let pool_conn = DatabaseConnection::init(env_vars.database_url()).pool;

    let config = Config::figment()
        .merge(("port", env_vars.port()))
        .merge(("address", env_vars.address()));

    rocket::build()
        .configure(config)
        .manage(pool_conn)
}
