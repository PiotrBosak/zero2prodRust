use std::net::TcpListener;

use once_cell::sync::Lazy;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;
use zerotoprod::configuration::{get_configuration, DatabaseName, DatabaseSettings};
use zerotoprod::telemetry::{get_subscriber, init_subscriber};

static TRACING: Lazy<()> = Lazy::new(|| {
    let subscriber = get_subscriber("test".into(), "debug".into());
    init_subscriber(subscriber);
});

#[tokio::test]
async fn health_check_works() {
    //Arrange
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    //Act
    let response = client
        .get(&format!("{}/health_check", &test_app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    //Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    //Arrange
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    //Act
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &test_app.address))
        .header("Content-type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let _saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&test_app.db_pool)
        .await
        .expect("Failed to fetch saved subscription");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    //Arrange

    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        //Act

        let response = client
            .post(&format!("{}/subscriptions", &test_app.address))
            .header("Content-type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");

        //Assert

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was{}",
            error_message
        );
    }
}
pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind port");
    let port = listener.local_addr().unwrap().port();
    let mut configuration = get_configuration().expect("Failed to read configuration");
    configuration.database.database_name = DatabaseName(Uuid::new_v4().to_string());
    let connection_pool = configure_database(&configuration.database).await;
    let server =
        zerotoprod::run(listener, connection_pool.clone()).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    let address = format!("http://localhost:{}", port);

    TestApp {
        address: address,
        db_pool: connection_pool,
    }
}
pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    //Create db
    let mut connection = PgConnection::connect(&config.connection_string_without_db())
        .await
        .expect("Failed to connect to postgres");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name.0).as_str())
        .await
        .expect("Failed to crate datbase");

    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to Postgres");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to run the database");

    connection_pool
}
