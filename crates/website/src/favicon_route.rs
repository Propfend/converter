use actix_web::{get, web, HttpResponse, Responder};

const FAVICON: &[u8] = include_bytes!("../static/favicon.ico.png");

pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(responder);
}

#[get("/favicon.ico")]
async fn responder() -> impl Responder {
    HttpResponse::Ok()
        .content_type("image/png")
        .insert_header(("X-Service-Worker-Cache", "store"))
        .body(FAVICON)
}
