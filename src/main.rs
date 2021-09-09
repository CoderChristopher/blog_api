use std::net::TcpListener;

use blog_api::startup::run_it;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	let listener = TcpListener::bind("127.0.0.1:0")
		.expect("Unable to bind a random port!");
	println!("Listening on port: {}",listener.local_addr().unwrap().port());
	run_it(listener)?.await
}
