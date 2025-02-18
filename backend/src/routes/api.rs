use actix_web::{web, HttpResponse, Responder};
use serde_json::json;

/// API endpoint: Returns "Hello, world!" as JSON
async fn test() -> impl Responder {
  HttpResponse::Ok().json(json!({"message": "Hello, world!"}))
}

/// Register user routes
pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.service(
      web::scope("/api")
          .route("get/test", web::get().to(test))  // GET /api/users
  );
}