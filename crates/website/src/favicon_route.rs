#[get("/favicon.ico")]
async fn respond() -> impl Responder {
    HttpResponse::Ok()
        .content_type("image/png")
        .insert_header(("X-Service-Worker-Cache", "store"))
        .body(FAVICON)
}
