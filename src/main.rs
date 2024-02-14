use actix_real_world::{config, run::build};

/// main instantiates the configuration and build the server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let conf = config::Settings::new().expect("Failed to read config file");
    let server = build(&conf).await?;

    server.await?;

    Ok(())
}
