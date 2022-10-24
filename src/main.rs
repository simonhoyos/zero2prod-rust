use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Create a new span storage.
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);

    // Set the span storage as the global default subscriber.
    init_subscriber(subscriber);

    // Get configuration from configuration.yml or panic if there's no file or file doesn't
    // include the required fields.
    let configuration = get_configuration().expect("Failed to read configuration.");

    // PgPool is an ARC allowing the application to span accross multiple cores and each of them
    // using their own reference.
    // Connect to Postgres using a generated url.
    // With secrecy we can prevent values from being exposed in our logs. To be able to read the
    // values in the code we need to expose them.
    let connection_pool =
        PgPool::connect(&configuration.database.connection_string().expose_secret())
            .await
            .expect("Failed to connect to Postgres.");

    // Create an address string using the port from our configuration.
    let address = format!("127.0.0.1:{}", configuration.application_port);

    // Set a listener to our address string
    let listener = TcpListener::bind(address)?;

    // Start the application
    run(listener, connection_pool)?.await
}
