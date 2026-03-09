mod presentation;
mod application;
mod domain;
mod data;
mod infrastructure;

use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use infrastructure::{config::Config, migrate};
use sqlx::postgres::PgPoolOptions;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init();

    let cfg = Config::from_env().expect("invalid config");

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&cfg.database_url)
        .await
        .expect("failed to connect to database");

    // миграции
    migrate::run(&pool).await.expect("migrations failed");



    let addr = format!("{}:{}", cfg.host, cfg.port);
    println!("→ listening on http://{}", addr);

    // 2. Клонируем cors ВНУТРИ замыкания для каждого воркера
    HttpServer::new(move || {
        let cors = Cors::default()
        .allowed_origin(&cfg.cors_origin)
        .allowed_methods(vec!["GET","POST","OPTIONS"])
        .allowed_headers(vec![
            actix_web::http::header::CONTENT_TYPE,
            actix_web::http::header::AUTHORIZATION,
        ])
        .supports_credentials()
        .max_age(600);
        App::new()
            .wrap(Logger::default())
            .wrap(cors)  // ✅ Cors::clone() работает напрямую
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(cfg.clone()))
            .configure(presentation::routes::configure)
    })
    .bind(addr)?
    .run()
    .await
}