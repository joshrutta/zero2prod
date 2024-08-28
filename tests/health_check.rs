use std::net::TcpListener;

use diesel::prelude::*;
use zero2prod::{configuration::get_configuration, startup::run};
use reqwest;
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};
use zero2prod::models::Subscription;
use zero2prod::schema::subscriptions::dsl::*;


#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request");

    // Assert 
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_200_for_valid_form_data() {
    // Arrange
    let address = spawn_app();
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_string = configuration.database.connection_string();
    // create an async connection
    let mut connection = AsyncPgConnection::establish(&connection_string)
        .await
        .expect("Failed to connect to database");

    let client = reqwest::Client::new();

    // Act
    let body = "name=test%20name&email=test%40test.com";
    let response = client
        .post(&format!("{}/subscriptions", &address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, response.status().as_u16());

    let result = subscriptions
        .select(Subscription::as_select())
        .limit(1)
        .load(&mut connection)
        .await
        .expect("Failed to fetch saved subscription");

    let saved: Option<&Subscription> = result.get(0);
    assert_eq!(saved.is_some(), true);
    let saved = saved.unwrap();

    assert_eq!("test@test.com", saved.email);
    assert_eq!("test name", saved.name);

}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=test%20name", "missing the email"),
        ("email=test%40test.com", "missing the name"),
        ("", "missing the email and name")
    ];

    // Act
    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to bind address");
    tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}