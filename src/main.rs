use actix_web::{App, Responder, HttpServer, HttpResponse, get, post, web};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use serde_json::{json};
use serde::{Deserialize};
use std::env;

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().json(json!({"health": "ok"}))
}

#[derive(Deserialize)]
struct AnalyzeData {
    pub url: String,
}

#[post("/analyze")]
async fn analyze(web::Json(item): web::Json<AnalyzeData>) -> impl Responder {
    println!("{}", item.url);
    HttpResponse::Ok().json(json!({"success": "ok"}))
}

fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(health)
            .service(analyze)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
