use rocket::{
    Request, Response,
    fairing::{Fairing, Info, Kind},
    http::Header,
};

use crate::ENV_VARS;

pub struct CorsConfig;

#[rocket::async_trait]
impl Fairing for CorsConfig {
    fn info(&self) -> rocket::fairing::Info {
        Info {
            name: "CORS configuration for api",
            kind: Kind::Response,
        }
    }
    async fn on_response<'a>(&self, _req: &'a Request<'_>, res: &mut Response<'a>) {
        res.set_header(Header::new("Access-Control-Allow-Origin", ENV_VARS.cors_allowed_origin()));
        res.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "GET, POST, PATCH, DELETE, OPTIONS",
        ));
        res.set_header(Header::new(
            "Access-Control-Allow-Headers",
            "Content-type, Authorization",
        ));
        res.set_header(Header::new("Access-Control-Allow-Credential", "true"));
    }
}
