use actix_web::{get, web, HttpResponse, Responder};
use rust_embed::{Embed};

pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(responder);
}

#[derive(Embed)]
#[folder = "static/"]
struct Asset;

#[get("/static/{path:.*}")]
async fn responder(path: web::Path<String>) -> impl Responder {
    let path = path.into_inner().to_string();

    match Asset::get(&path) {
        Some(file) => HttpResponse::Ok().body(file.data),

        None => HttpResponse::NotFound().body("File not found"),
    }
}
