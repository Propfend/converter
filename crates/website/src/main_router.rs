use actix_web::{get, web, HttpResponse};
use askama::Template;
use askama_resolver;

#[derive(Template)]
#[template(path = "index.html")]
struct Product<'a> {
    name: &'a String,
}

pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(main_page);
}

#[get("/")]
async fn main_page() -> HttpResponse {
    let product = Product { 
        name: &String::from("Miguel"), 
    };

    askama_resolver::into_response(&product)
}   