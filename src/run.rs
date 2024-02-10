use std::net::TcpListener;

use actix_web::{dev::Server, App, HttpServer};

use crate::routes::health;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let s = HttpServer::new(|| App::new().service(health))
        .listen(listener)?
        .run();
    Ok(s)
}
