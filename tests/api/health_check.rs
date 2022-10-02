use crate::helpers::spawn_app;

#[tokio::test]
async fn health_check_works() {
    // Start test application
    let test_app = spawn_app().await;

    // Create a reqwest client.
    let client = reqwest::Client::new();

    // Simulate request using our client.
    let response = client
        .get(&format!("{}/health_check", &test_app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
