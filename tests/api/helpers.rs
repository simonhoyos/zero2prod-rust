use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;
use zero2prod::configuration::{get_configuration, DatabaseSettings};
use zero2prod::startup::run;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Connect to Postgres. Database is not require in this case as we are creating a new logical
    // database on every test execution.
    let mut connection = PgConnection::connect(&config.connection_string_without_db())
        .await
        .expect("Failed to connect to Postgres.");

    // Create the logical database.
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database.");

    // Create a connection pool (ARC) to the logical database.
    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    // Run migrations. This is equivalent to running the migrations from sqlx-cli.
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database.");

    // Return the connection pool
    connection_pool
}

pub async fn spawn_app() -> TestApp {
    // Create a listener in a random port for testing purposes.
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

    // Get the port got in the previous step.
    let port = listener.local_addr().unwrap().port();

    // Create an address with the port.
    let address = format!("http://127.0.0.1:{}", port);

    // Get configuration from configuration file.
    let mut configuration = get_configuration().expect("Failed to read configuration.");

    // Logical databases: For every tests suit execution we create a new database instead of using
    // the same on every time. This is to prevent test collision.
    // All databases can be deleted at once by removing the container. So it is easy to cleanup.
    configuration.database.database_name = Uuid::new_v4().to_string();

    // Create a connection pool to the logical database and configure everything neccessary to make
    // it work.
    let connection_pool = configure_database(&configuration.database).await;

    // Run the test application.
    let server = run(listener, connection_pool.clone()).expect("Failed to bind address");

    // Allow sever to run concurrently.
    tokio::spawn(server);

    // Return the test application.
    TestApp {
        address,
        db_pool: connection_pool,
    }
}
