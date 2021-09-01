use reqwest::*;
use tokio::*;
use std::net::TcpListener;

async fn check_health_test() {
	let address = spawn_app();
	let client = reqwest::Client::new();

	let response = client
		.get(&format!("{}/health_check",&address))
		.send()
		.await
		.expect("Failed to make a good request!");
	assert!(response.status().is_success());
	assert_eq!(Some(0),response.content_length());
}

fn spawn_app() -> String {
	let listener = TcpListener::bind("127.0.0.1")
		.expect("Could not bind a random address!");
	let port = listener.local_addr().unwrap().port();
	let server = blog_api::run_it(listener).expect("Failed to listen");
	let _ = tokio::spawn(server);
	format!("127.0.0.1:{}",port)
}
