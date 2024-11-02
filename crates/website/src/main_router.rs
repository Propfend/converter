use actix_files::NamedFile;
use actix_web::{get, web, HttpResponse, Responder, Result};
use tera::{Tera, Context};
use std::path::PathBuf;
use lazy_static::lazy_static;
use askama::Template;

const FAVICON: &[u8] = include_bytes!("../static/favicon.ico.png");

#[derive(Template)]
#[template(path = "index.html")]
struct Product<'a> {
    name: &'a String,
}

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("examples/basic/templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![".html", ".sql"]);
        tera
    };
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