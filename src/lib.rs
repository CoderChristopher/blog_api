use actix_web::{HttpRequest, HttpResponse, HttpServer, App, web};
use actix_web::dev::Server;
use std::net::TcpListener;
//use serde::Deserialize;

//fn establish_connection() -> mysql::MysqlConnection {
//}

fn health_check(req: HttpRequest) -> HttpResponse {
	HttpResponse::Ok().finish()
}

fn query_titles(req: HttpRequest) -> HttpResponse {
	HttpResponse::Ok().finish()
}

pub fn run_it(listener: TcpListener) -> Result<Server, std::io::Error> {
	let server = HttpServer::new(|| {
		App::new()
			.route( "/health_check", web::get().to(health_check))
			.route( "/query_titles", web::get().to(health_check))
		
	})
	.listen(listener)?
	.run();
	Ok(server)
}
