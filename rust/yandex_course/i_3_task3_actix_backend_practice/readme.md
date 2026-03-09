# PostgreSQL
```sh
sudo apt update
sudo apt install postgresql-14 postgresql-client-14
```
# Запуск и автозапуск
```sh
sudo systemctl start postgresql
sudo systemctl enable postgresql
```
# Создать БД и задать пароль суперпользователю postgres
```sh
sudo -u postgres psql -c "CREATE DATABASE practice_db;"
sudo -u postgres psql -c "ALTER USER postgres PASSWORD 'postgres';"
```

Если psql спрашивает пароль — задайте пароль командой:
```sh 
sudo -u postgres psql -c "ALTER USER postgres WITH PASSWORD 'postgres';"
```

. rust/yandex_course/i_3_task3_actix_backend_practice/.env
Запуск:
```sh
export DATABASE_URL=postgres://postgres:postgres@127.0.0.1:5432/practice_db
export HOST=127.0.0.1
export PORT=8080

# лайтовый секрет
export JWT_SECRET=dev_super_secret_change_me_please_32_bytes_min

# дев-ориджин для CORS
export CORS_ORIGIN=http://localhost:3000
cargo run --package i_3_task3_actix_backend_practice
```



Проверка запуска
```sh
curl -s http://127.0.0.1:8080/health 
```