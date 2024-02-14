use std::net::TcpListener;

use actix_web::{dev::Server, App, HttpServer};

use crate::{config::Settings, routes::health};

/// build is responsible to start all services and call the server runner
pub async fn build(conf: &Settings) -> Result<Server, std::io::Error> {
    let addr = format!("{}:{}", conf.server.host, conf.server.port);
    let listener = TcpListener::bind(addr)?;

    run(listener)
}

/// run only runs the server with the services provided as parameters
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let s = HttpServer::new(|| App::new().service(health))
        .listen(listener)?
        .run();
    Ok(s)
}
