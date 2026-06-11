Старт сервера
```sh 
 make i_3_task_11_actix_jwt_demo
```

✅ Шаг 2: Запрос к защищённому маршруту с токеном
```sh 
# curl -s -X POST http://127.0.0.1:8080/login \
#   -H "Content-Type: application/json" \
#   -d '{"username":"admin","password":"password123"}'

TOKEN_DEMO=$(curl -s -X POST http://127.0.0.1:8080/login \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","password":"password123"}'\
  | jq -r '.access_token')


echo $TOKEN_DEMO
```

❌ Шаг 3: Попытка доступа без токена
```sh 
curl -s http://127.0.0.1:8080/protected
```


```sh 
curl -s http://127.0.0.1:8080/api/protected \
  -H "Authorization: Bearer $TOKEN_DEMO"
```