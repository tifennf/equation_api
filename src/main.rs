use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use equation_generator::basic;
use std::env;



#[get("/")]
async fn index() -> impl Responder {
	let body = serde_json::json!([]);

    HttpResponse::Ok().body(body)
}

//return a mix of equation
#[get("/equation")]
async fn equation() -> impl Responder {
	let equation_list = basic::generate_two_factor(1, 0..100);

	let body = serde_json::to_string(&equation_list).unwrap();
	
	HttpResponse::Ok().body(body)
}

//return equation 2f
#[get("/equation/two-factor")]
async fn two_factor() -> impl Responder {
	let equation_list = basic::generate_two_factor(1, 0..100);

	let body = serde_json::to_string(&equation_list).unwrap();
	
	HttpResponse::Ok().body(body)
}

//return equation 3f
#[get("/equation/tree-factor")]
async fn three_factor() -> impl Responder {
	let equation_list = basic::generate_two_factor(1, 0..100);

	let body = serde_json::to_string(&equation_list).unwrap();
	
	HttpResponse::Ok().body(body)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {

	let port = env::var("PORT").unwrap_or("3000".to_string());
	let ip = "0.0.0.0";
	
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(equation)
    })
    .bind(format!("{}:{}", ip, port))?
    .run()
    .await
}
