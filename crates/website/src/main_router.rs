use actix_files::NamedFile;
use actix_web::{get, web, Result};
use std::path::PathBuf;
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct Product<'a> {
    name: &'a String,
}

pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(main_page);
}

#[get("/")]
async fn main_page() -> Result<NamedFile> {
    let product = Product { 
        name: &String::from("Miguel"), 
    }; 

    println!("{}", product.render().unwrap());
    
    let path: PathBuf = "../templates/index.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}