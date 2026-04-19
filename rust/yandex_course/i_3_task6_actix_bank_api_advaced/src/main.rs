mod domain;
mod application;
mod data;
mod presentation;
mod infrastructure;

use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use application::bank_service::BankService;
use data::account_repository::InMemoryAccountRepository;
use infrastructure::logging::init_logging;
use presentation::handlers;
use presentation::middleware::{RequestIdMiddleware, TimingMiddleware};


fn configure_cors() -> Cors {
    let allowed_origins: Vec<String> = std::env::var("ALLOWED_ORIGINS")
        .unwrap_or_else(|_| "http://localhost:8080".to_string())
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();
    
    let mut cors = Cors::default()
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
        .allowed_headers(vec![
            actix_web::http::header::CONTENT_TYPE,
            actix_web::http::header::AUTHORIZATION,
        ])
        .expose_headers(vec!["X-Total-Count"]) // пример полезного expose
        .max_age(3600);
    
    // Добавляем каждый origin отдельно (Cors не поддерживает вектор напрямую)
    for origin in allowed_origins {
        cors = cors.allowed_origin(&origin);
    }
    
    cors
} 

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_logging();

    let repo = Arc::new(InMemoryAccountRepository::default());
    let service = BankService::new(Arc::clone(&repo));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(RequestIdMiddleware)
            .wrap(TimingMiddleware)
            // .wrap(Cors::permissive())
            .wrap(configure_cors()) 
            .app_data(web::Data::new(service.clone()))
            .configure(handlers::configure)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

