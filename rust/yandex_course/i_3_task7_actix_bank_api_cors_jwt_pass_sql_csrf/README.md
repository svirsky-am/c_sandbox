## Lesson 3.2.5 — Bank API Security


https://practicum.yandex.ru/learn/middle-rust-blockchain/courses/576cf632-753a-4dac-acf5-7e7667b820be/sprints/719747/topics/658dc6c8-1968-45a4-a36e-6e616d9e322d/lessons/25723981-7f92-460c-8a7c-b2c2a80a412e/
Продолжение Bank API: добавлены защита CORS, JWT‑авторизация, Argon2 для паролей, кастомные middleware и ошибки.


```sh 
   export HOST=127.0.0.1
   export PORT=8080
   export JWT_SECRET=dev_super_secret_change_me_please
   export  CORS_ORIGINS=http://localhost:3000

make i_3_task7_actix_bank_api_cors_jwt_pass_sql_csrf
```
Проверка 
Регистрация:
```sh
curl -X POST http://localhost:8080/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{"email": "user@example.com", "password": "secure123"}' 
```
Логин:
```sh 
curl -X POST http://localhost:8080/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email": "user@example.com", "password": "secure123"}'

```
Получаем JWT-токен и Создание счёта с JWT:
```sh
export TOKEN="eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI3Zjk4OTI4Zi01M2I1LTRhODAtODY0NS1hMzlkNjIyZmM5MjQiLCJleHAiOjE3NzcxMjUzOTIsImlhdCI6MTc3NzEyMTc5Mn0.RR6vmHk6wcNMyIwkO3K3VvPZ7S3yAOA_fxUX-hTdFwc"


TOKEN=$(curl -X POST http://localhost:8080/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email": "user@example.com", "password": "secure123"}'\
  | jq -r '.access_token')

```
c# 4. Создание счёта с JWT
```sh
curl -X POST http://localhost:8080/api/accounts \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{"id": 1, "initial": 1000}'


```sh
curl -X POST http://localhost:8080/api/accounts \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI3Zjk4OTI4Zi01M2I1LTRhODAtODY0NS1hMzlkNjIyZmM5MjQiLCJleHAiOjE3NzcxMjUzOTIsImlhdCI6MTc3NzEyMTc5Mn0.RR6vmHk6wcNMyIwkO3K3VvPZ7S3yAOA_fxUX-hTdFwc" \
  -d '{"id": 1, "initial": 1000}'
# Ответ: {"id": 1}
```
# 5. Проверка баланса
```sh
curl -X GET http://localhost:8080/api/accounts/1 \
  -H "Authorization: Bearer $TOKEN"
# Ответ: {"id": 1, "balance": 1000}
```
# 6. Попытка доступа без токена (должна вернуть 401)

```sh
curl -X GET http://localhost:8080/api/accounts/1
# Ответ: {"error": "missing bearer"} 
```

### Запуск

1. Создай `.env` или экспортируй переменные:
   ```
   HOST=127.0.0.1
   PORT=8080
   JWT_SECRET=dev_super_secret_change_me_please
   CORS_ORIGINS=http://localhost:3000
   ```
2. Запусти сервер:
   ```bash
   cargo run
   ```

### Что реализовано

- `RequestIdMiddleware`, `TimingMiddleware`, `JwtAuthMiddleware`.
- Регистрация и логин с Argon2 + JWT.
- Все банковские действия защищены токеном, `AuthenticatedUser` берётся из extensions.
- Гибкий CORS и security headers.
- Единая система ошибок `BankError` → JSON.

