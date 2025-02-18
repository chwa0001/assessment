mod routes;

use actix_files as fs;
use actix_web::{web, App, HttpServer, HttpResponse, Responder, Result};
use tera::{Tera, Context};
use routes::api;
use std::path::PathBuf;

async fn index(tera: web::Data<Tera>) -> impl Responder {
    let mut context = Context::new();
    context.insert("api_url", "https://api.example.com");

    let rendered = tera.render("index.html", &context).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

async fn default_index() -> Result<fs::NamedFile> {
    let path: PathBuf = "build/index.html".parse().unwrap();
    Ok(fs::NamedFile::open(path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tera = Tera::new("build/*.html").expect("Failed to load templates");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tera.clone()))
            .configure(api::config)
            // Route root to the templated index.html
            // .route("/api/get", web::get().to(test))
            .route("/", web::get().to(index))
            // Serve other static files (JS, CSS, images)
            .service(fs::Files::new("/", "build").index_file("index.html"))
            // Catch-all route: Serve index.html for React Router
            .default_service(web::get().to(default_index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}