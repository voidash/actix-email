//! tests/health_check.rs
use std::net::TcpListener;

#[actix_rt::test]
async fn health_check_test() {
    let address = spawn_app();
    println!("{}/health_check", address);

    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health_check", address))
        .send()
        .await
        .expect("failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind address");
    let port = listener.local_addr().unwrap().port();
    let server = z2p::run(listener).expect("failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
