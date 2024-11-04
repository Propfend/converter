use actix_web::{get, web, HttpResponse};
use askama::Template;
use askama_resolver;

#[derive(Template)]
#[template(path = "index.html")]
struct LlamaCppResponse<'a> {
    token: &'a String,
}

pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(main_page);
}

#[get("/")]
async fn main_page() -> HttpResponse {
    let llamacpp_response = LlamaCppResponse { 
        token: &String::from("Miguel"), 
    };

    askama_resolver::into_response(&llamacpp_response)
}   