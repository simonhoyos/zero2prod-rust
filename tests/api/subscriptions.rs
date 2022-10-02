use crate::helpers::spawn_app;
use sqlx;

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Start test application
    let test_app = spawn_app().await;

    // Create a reqwest client
    let client = reqwest::Client::new();

    // Mock request body
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    // Simulate request using our client.
    let response = client
        .post(&format!("{}/subscriptions", &test_app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    // NOT OPTIMAL perform a sql query to get the recently created subscription to validate whether
    // or not it was created.
    // For better consistency while testing we can create second endpoint where we get the
    // subscription.
    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&test_app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Start test application.
    let test_app = spawn_app().await;

    // Create a reqwest client.
    let client = reqwest::Client::new();

    // Mock request data.
    // In this case we have an Enum with the body and a custom message for every test.
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursule_le_guin%40gmail.com", "missing the name"),
        ("", "missing both"),
    ];

    // Test multiple cases on a single test
    for (invalid_body, error_message) in test_cases {
        // Simulate request using our client.
        let response = client
            .post(&format!("{}/subscriptions", &test_app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        // A custom message can be set on our assertions so we can easily identify which tests are
        // failing.
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}
