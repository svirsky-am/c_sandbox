Есть урок [Спринт 3/7: 3 «Асинхронные веб-приложения и CLI-инструменты на Rust» → Тема 3/6: Rust backend → Урок 5/7](
 https://practicum.yandex.ru/learn/middle-rust-blockchain/courses/576cf632-753a-4dac-acf5-7e7667b820be/sprints/719747/topics/658dc6c8-1968-45a4-a36e-6e616d9e322d/lessons/25723981-7f92-460c-8a7c-b2c2a80a412e/)


В конце урока  можно скачать  решение задачек по бекэнду  
 https://code.s3.yandex.net/middle-java/lesson-3.2.5-bank-api-security.rar

Я скачиваю его , распаковываю и далее:
```sh
sudo systemctl start postgresql.service 
```

```sh 
   export HOST=127.0.0.1
   export PORT=8080
   export JWT_SECRET=dev_super_secret_change_me_please
   export  CORS_ORIGINS=http://localhost:3000
```
Запускаю бек
```sh 
cargo run --package bank-api-security
```
 Авторизовываюсь и получаю тикет 
 ```sh
curl -X POST http://localhost:8080/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{"email": "user@example.com", "password": "secure123"}' 

 export TOKEN_CLIENT=$(curl -X POST http://localhost:8080/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email": "user@example.com", "password": "secure123"}' \
  | jq -r '.access_token') 

  echo "Токен получен (первые 20 символов): ${TOKEN_CLIENT:0:20}..."
 ```

 Далее пытаюсь завести счет:
```sh 
curl -X POST http://localhost:8080/api/accounts \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN_CLIENT" \
  -d '{"id": 1, "initial": 1000}'
  ```

Ответ: 404.  В логах 
```
{"timestamp":"2026-04-27T19:49:27.981133022Z","level":"INFO","fields":{"message":"request completed","method":"POST","path":"/api/accounts","status":404,"duration_ms":"0"}}
```