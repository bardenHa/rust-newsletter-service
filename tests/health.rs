#[tokio::test]
async fn health_check_works() {
    spawn_app();

    let client = reqwest::Client::new();
    let res = client
        .get("http://127.0.0.1:8000/health")
        .send()
        .await
        .expect("Failed to send request");

    assert!(res.status().is_success());
    assert_eq!(Some(0), res.content_length());
}

fn spawn_app() {
    let server = zero2prod::run().expect("Failed to bind address");
    let _ = tokio::task::spawn(server);
}
