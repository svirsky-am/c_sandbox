use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    web, App, HttpServer, HttpResponse, HttpRequest, HttpMessage
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::future::{ready, Future, Ready};
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{SystemTime, UNIX_EPOCH};
use std::rc::Rc;

const SECRET: &str = "super_secret_key_change_in_production";

// Структура JWT payload (claims)
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Claims {
    sub: String, // subject (например, username или user_id)
    exp: usize,  // expiration timestamp (в секундах)
}

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

// 🔐 Эндпоинт авторизации (НЕ защищён мидлварью)
async fn login(body: web::Json<LoginRequest>) -> HttpResponse {
    // Упрощённая проверка (в реальности: хеш пароля + БД)
    if body.username != "admin" || body.password != "password123" {
        return HttpResponse::Unauthorized().json(serde_json::json!({ "error": "Invalid credentials" }));
    }

    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as usize;
// токен живёт 1 час
    let claims = Claims { sub: body.username.clone(), exp: now + 3600 };

    match encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET.as_bytes())) {
        Ok(token) => HttpResponse::Ok().json(serde_json::json!({ "access_token": token })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({ "error": e.to_string() })),
    }
}

// 🛡️ Защищённый эндпоинт (Claims берутся из extensions, поставленных мидлварью)
async fn protected(req: HttpRequest) -> HttpResponse {
    match req.extensions().get::<Claims>() {
        Some(claims) => HttpResponse::Ok().json(serde_json::json!({
            "message": "Access granted",
            "user": claims.sub,
            "expires_at": claims.exp
        })),
        None => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Middleware failed to inject claims"
        })),
    }
}

// ================= MIDDLEWARE =================

pub struct JwtMiddleware;

impl<S, B> Transform<S, ServiceRequest> for JwtMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type InitError = ();
    type Transform = JwtMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtMiddlewareService { service: Rc::new(service) }))
    }
}

pub struct JwtMiddlewareService<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for JwtMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let srv = self.service.clone();

        Box::pin(async move {
            // 1. Извлекаем заголовок
            let auth_header = req.headers().get("Authorization");
            let token = match auth_header.and_then(|h| h.to_str().ok()) {
                Some(s) => s.strip_prefix("Bearer ").unwrap_or(s),
                None => return Err(actix_web::error::ErrorUnauthorized("Missing or invalid Authorization header")),
            };

            // 2. Валидируем токен
            let token_data = decode::<Claims>(
                token,
                &DecodingKey::from_secret(SECRET.as_bytes()),
                &Validation::default(), // автоматически проверяет exp, alg и т.д.
            ).map_err(|e| actix_web::error::ErrorUnauthorized(format!("Invalid token: {}", e)))?;

            // 3. Кладём claims в extensions, чтобы хендлер мог их достать
            req.extensions_mut().insert(token_data.claims);

            // 4. Передаём управление дальше
            let res = srv.call(req).await?;
            Ok(res)
        })
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Сервер: http://127.0.0.1:8080");
    
    HttpServer::new(|| {
        App::new()
            // /login доступен всем
            .route("/login", web::post().to(login))
            // Защищённый скоуп: мидлварь сработает только для /protected
            .service(
                web::scope("/api")
                    .wrap(JwtMiddleware)
                    .route("/protected", web::get().to(protected))
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}