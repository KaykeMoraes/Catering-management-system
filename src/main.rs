use crate::config::Config;

mod config;

#[macro_use]
extern crate log;

#[rocket::launch]
async fn launch() -> _ {
    env_logger::init();
    let config = Config::init().to_rocket();
    rocket::build().configure(config)
}
