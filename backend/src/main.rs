mod routes;

use actix_files as fs;
use actix_web::{web, HttpResponse, Responder, Result,web::ServiceConfig};
use shuttle_actix_web::ShuttleActixWeb;
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

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let tera = Tera::new("build/*.html").expect("Failed to load templates");

    let config = move |cfg: &mut ServiceConfig| {
        cfg.app_data(web::Data::new(tera.clone()))
        .configure(api::config)
        .service(fs::Files::new("/", "build").index_file("index.html")) // Serve static files
        .default_service(web::get().to(index));
    };
    
    Ok(config.into())
}