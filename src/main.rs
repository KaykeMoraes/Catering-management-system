use crate::config::Config;

mod config;

#[macro_use]
extern crate log;

#[rocket::launch]
async fn launch() -> _ {
    env_logger::init();
    let config = Config::init().to_figment();
    rocket::build().configure(config)
}
