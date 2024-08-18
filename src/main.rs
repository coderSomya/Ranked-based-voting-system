use actix_web::{web, App, HttpServer, HttpResponse};
use diesel::prelude::*;
use tera::Tera;

#[actiz_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
             .service(web::resource("/").to(index))
             .service(web::resource("/vote").to(submit_vote))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn index(tera: web::Data<Tera>) -> HttpResponse{
    let ctx = tera::Context::new();
    let rendered = tera.render("index.html", &ctx).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

async fn submit_vote() -> HttpResponse{
    //handle vote submission

    HttpResponse::Ok().body("Voted...")
}