mod authentication;

use actix_files as fs;
use actix_web::{
    dev::ServiceRequest,
    error::ErrorUnauthorized,
    web::{self, ServiceConfig},
    Error,
};
use actix_web_httpauth::{extractors::basic::BasicAuth, middleware::HttpAuthentication};
use authentication::Creds;
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::SecretStore;

async fn validator(
    req: ServiceRequest,
    credentials: BasicAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let creds = req.app_data::<web::Data<Creds>>().cloned().unwrap();
    if let Some(password) = credentials.password() {
        if credentials.user_id() == creds.username && password == creds.password {
            return Ok(req);
        }
    }
    Err((ErrorUnauthorized("nope"), req))
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    // Fetch basic auth credentials from Secrets.toml
    let creds = Creds::new(
        secrets.get("BASIC_USERNAME").unwrap(),
        secrets.get("BASIC_PASSWORD").unwrap(),
    );
    let config = move |cfg: &mut ServiceConfig| {
        cfg.app_data(web::Data::new(creds));
        cfg.service(
            web::scope("")
                .service(fs::Files::new("/", "static").index_file("index.html"))
                .wrap(HttpAuthentication::basic(validator)), // add filter to enable basic auth
        );
    };

    Ok(config.into())
}
