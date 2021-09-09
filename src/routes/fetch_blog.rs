use actix_web:: {HttpRequest, HttpResponse};
use serde::*;
//use actix_web::dev::HttpResponseBuilder;

pub fn query_titles(req: HttpRequest) -> HttpResponse {
	let response = HttpResponse::Ok()
		.body("SENIP");

	response
}
