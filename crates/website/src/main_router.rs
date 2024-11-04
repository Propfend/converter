use actix_web::{get, web, HttpResponse};
use askama::Template;
use askama_resolver;

pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(responder);
}

#[derive(Template)]
#[template(path = "index.html")]
struct LlamaCppResponse<'a> {
    token: &'a String,
}

// #[derive(Template)]
// #[template(path = "index.html")]
// struct Person {
//     name: String,
// }

#[get("/")]
async fn responder() -> HttpResponse {
    let llamacpp_response = LlamaCppResponse { 
        token: &String::from(
             "Lorem ipsum dolor sit amet, consectetur adipisicing elit. Fugit sit, laboriosam earum minus amet nam quibusdam iure? Ut necessitatibus, voluptatibus exercitationem esse quibusdam ipsum minus excepturi, veritatis accusantium soluta molestiae.
    "),
 };

    askama_resolver::into_response(&llamacpp_response)
}
