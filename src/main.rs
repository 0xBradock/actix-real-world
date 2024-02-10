use std::net::TcpListener;

use actix_real_world::{config, run::run};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let conf = config::Settings::new().expect("Failed to read config file");
    let addr = format!("{}:{}", conf.server.host, conf.server.port);
    let listener = TcpListener::bind(addr)?;

    run(listener)?.await
}
