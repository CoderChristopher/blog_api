use actix_web:: {HttpRequest, HttpResponse, web};

#[derive(serde::Serialize)]
struct BlogResponse {
	name: String,
	author: String,
	rating: u8,
}

#[derive(serde::Deserialize)]
pub struct BlogRequest {
	id: u8
}

pub fn query_titles(req: web::Form<BlogRequest>) -> HttpResponse {
	println!("id:{}",req.id);
	let json = BlogResponse{ 
			name: String::from("First Blog"),
			author: String::from("Chris Copeland"),
			rating: 5,
		};
	let response = HttpResponse::Ok()
		.json(json);
	response
}
