```sh 
sudo systemctl start postgresql.service
sudo -u postgres psql  -c "CREATE DATABASE bank_api"
bank_api\


```

пример ошибки из урока 
thread 'main' panicked at rust/yandex_course/i_3_task9-bank-api-postgres/src/main.rs:32:10:
failed to connect to database: Database(PgDatabaseError { severity: Fatal, code: "3D000", message: "database \"bank_api\" does not exist", detail: None, hint: None, position: None, where: None, schema: None, table: None, column: None, data_type: None, constraint: None, file: Some("postinit.c"), line: Some(888), routine: Some("InitPostgres") })
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
make: *** [rust/rust.mk:135: i_3_task9-bank-api-postgres] Error 101
```sh 

export HOST=127.0.0.1
export PORT=8080
export DATABASE_URL=postgres://postgres:postgres@127.0.0.1:5432/bank_api
export JWT_SECRET=dev_super_secret_change_me_please
export CORS_ORIGINS=http://localhost:3000
export EXCHANGE_API_URL=https://api.exchangerate-api.com/v4/latest

# После создания счёта через API, проверьте в БД:
sudo -u postgres psql -d bank_db -c "SELECT * FROM accounts;"

# После депозита проверьте обновление:
sudo -u postgres psql -d bank_db -c "SELECT id, balance, updated_at FROM accounts WHERE id = 1;" 

```
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
```sh 
export TOKEN=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJmODYyNjI0Yy1mZTJkLTQxOTgtOWJmYy03ODUyYWNiYzIwMWEiLCJleHAiOjE3NzY1NTIzMjgsImlhdCI6MTc3NjU0ODcyOH0.R9bTdPPoIeQcEWTPG1vC7zo4ijRd4wDkMepbPZsHjC8

# Создание счёта
curl -X POST http://127.0.0.1:8080/api/accounts \
  -H "Authorization: Bearer $YOUR_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"initial_balance": 1000}'

# Получение баланса
curl http://127.0.0.1:8080/api/accounts/1 \
  -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJmODYyNjI0Yy1mZTJkLTQxOTgtOWJmYy03ODUyYWNiYzIwMWEiLCJleHAiOjE3NzY1NTIzMjgsImlhdCI6MTc3NjU0ODcyOH0.R9bTdPPoIeQcEWTPG1vC7zo4ijRd4wDkMepbPZsHjC8"

# Получение курса валют (реальный API, возвращает актуальные курсы)
curl http://127.0.0.1:8080/api/exchange/USD/EUR \
  -H "Authorization: Bearer YOUR_TOKEN"
# Ответ: {"from":"USD","to":"EUR","rate":0.865}

curl http://127.0.0.1:8080/api/exchange/USD/RUB \
  -H "Authorization: Bearer YOUR_TOKEN"
# Ответ: {"from":"USD","to":"RUB","rate":80.95} 

```



## Lesson 3.2.6 — Bank API Postgres

Продолжение Bank API: вместо in-memory хранилища используется PostgreSQL через `sqlx`, добавлен сервис актуальных курсов валют.

### Подготовка

1. Создай `.env` на основе `env.example`.
2. Подними PostgreSQL и создай базу `bank_api`.
3. Запусти миграции автоматически при старте (`sqlx::migrate!()`).

### Запуск

```bash
cargo run
```

Сервер слушает `http://127.0.0.1:8080`. Все маршруты `/api/*` сохранены (регистрация, логин, операции со счётами) и теперь читают/пишут реальные данные в БД. Endpoint `/api/exchange/{from}/{to}` берёт курсы с внешнего API.

