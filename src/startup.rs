use actix_web::dev::Server;
use actix_web::{App, web, HttpServer, };
use std::net::TcpListener;

use crate::routes::{ health_check, fetch_blog};
use crate::routes::fetch_blog::query_titles;

pub fn run_it(listener: TcpListener) -> Result<Server, std::io::Error> {
	let server = HttpServer::new(|| {
		App::new()
			.route( "/health_check", web::get().to(health_check))
			.route( "/query_titles", web::get().to(query_titles))
		
	})
	.listen(listener)?
	.run();
	Ok(server)
}
