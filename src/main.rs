use std::sync::LazyLock;

use rocket::Config;

use crate::{
    config::{EnvironmentVariables, cors::CorsConfig},
    repository::DatabaseConnection,
};

mod config;
mod model;
mod repository;
mod utils;

#[macro_use]
extern crate log;

pub static ENV_VARS: LazyLock<EnvironmentVariables> =
    LazyLock::new(|| EnvironmentVariables::load());

#[rocket::launch]
async fn launch() -> _ {
    env_logger::init();

    let pool_conn = DatabaseConnection::init(ENV_VARS.database_url()).pool;

    let config = Config::figment()
        .merge(("port", ENV_VARS.port()))
        .merge(("address", ENV_VARS.address()));

    rocket::build()
        .configure(config)
        .attach(CorsConfig)
        .manage(pool_conn)
}
