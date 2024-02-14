use actix_real_world::{config, run::build};

async fn spawn_app() -> String {
    let conf = config::Settings::new().expect("Failed to read config file");
    let server = build(&conf).await;
    match server {
        Ok(server) => tokio::spawn(server),
        Err(_) => panic!("Failed to create server"),
    };

    format!("http://{}:{}", conf.server.host, conf.server.port)
}

#[actix_web::test]
async fn health_check_works() {
    // Arrange
    let server = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health", server))
        .send()
        .await
        .expect("Failed to execute response");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
