use actix_files::NamedFile;
use actix_web::{get, web, HttpResponse, Responder, Result};
use std::path::PathBuf;

const FAVICON: &[u8] = include_bytes!("../static/favicon.ico.png");

pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(main_page);
}

#[get("/")]
async fn main_page() -> Result<NamedFile> {
    let path: PathBuf = "./static/index.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}

#[get("/favicon.ico")]
async fn respond() -> impl Responder {
    HttpResponse::Ok()
        .content_type("image/png")
        .insert_header(("X-Service-Worker-Cache", "store"))
        .body(FAVICON)
}
