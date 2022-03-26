use actix_web::{App, Responder, HttpServer, HttpResponse, get, post, web};
use serde_json::{json};

use super::models::*;
use super::ps_adapter::*;

#[get("/health")]
pub async fn health() -> impl Responder {
    println!("health");
    HttpResponse::Ok().json(json!({"success": "ok"}))
}

#[post("/url")]
pub async fn get(web::Json(item): web::Json<UrlRequest>) -> impl Responder {
    println!("check");
    let url = query_url(item.id);
    HttpResponse::Ok().json(json!({
        "id": url.id,
        "url": url.url,
        "result": url.result,
        "status": url.status
    }))
}

#[post("/analyze")]
pub async fn analyze(web::Json(item): web::Json<AnalyzeRequest>) -> impl Responder {
    println!("set new url for analysis");
    insert_new_url(item.url);
    HttpResponse::Ok().json(json!({"success": "ok"}))
}

#[actix_web::main]
pub async fn start_api() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(health)
            .service(get)
            .service(analyze)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}