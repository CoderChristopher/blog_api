use actix_web::{ HttpResponse };

pub fn health_check() -> HttpResponse {
	HttpResponse::Ok().finish()
}
