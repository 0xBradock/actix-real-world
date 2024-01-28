use actix_web::{dev::Server, App, HttpServer};

use crate::routes::health;

pub fn run(host: String, port: u16) -> Result<Server, std::io::Error> {
    let s = HttpServer::new(|| App::new().service(health))
        .bind((host, port))?
        .run();
    Ok(s)
}
