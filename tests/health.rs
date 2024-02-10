// use actix_real_world::routes::health;

use std::net::TcpListener;

use actix_real_world::{config, run::run};

fn spawn_app() -> String {
    let conf = config::Settings::new().expect("Failed to read config file");

    let listener =
        TcpListener::bind(format!("{}:0", conf.server.host)).expect("Failed to bind to port");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to load server");
    let _ = tokio::spawn(server);

    format!("http://{}:{}", conf.server.host, port)
}

#[actix_web::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health", &address))
        .send()
        .await
        .expect("Failed to execute response");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
