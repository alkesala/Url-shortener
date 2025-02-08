use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Deserialize)]
struct CreateUrl {
    url: String,
}

#[derive(Serialize)]
struct ShortenedUrl {
    short_code: String,
}

struct AppState {
    urls: Mutex<HashMap<String, String>>,
}

#[post("/shorten")]
async fn shorten_url(data: web::Data<AppState>, url: web::Json<CreateUrl>) -> HttpResponse {
    let code = nanoid::nanoid!(6);
    let mut urls = data.urls.lock().unwrap();
    urls.insert(code.clone(), url.url.clone());
    HttpResponse::Ok().json(ShortenedUrl { short_code: code })
}

#[get("/{code}")]
async fn redirect(data: web::Data<AppState>, code: web::Path<String>) -> HttpResponse {
    let urls = data.urls.lock().unwrap();
    if let Some(url) = urls.get(&code.into_inner()) {
        HttpResponse::Found()
            .append_header(("Location", url.to_string()))
            .finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(AppState {
        urls: Mutex::new(HashMap::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(shorten_url)
            .service(redirect)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}