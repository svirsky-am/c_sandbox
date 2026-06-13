# Counter Dev-UI

Минимальный UI для ручной проверки смарт-контракта Counter.

## Что можно тестировать вручную

| Действие | Как |
|----------|-----|
| **Подключить кошелёк** | Кнопка «Connect Wallet» → Phantom (RPC должен быть Localhost 127.0.0.1:8899) |
| **Инициализировать счётчик** | Кнопка «Initialize» — создаётся PDA-аккаунт счётчика для текущего кошелька |
| **Увеличить счётчик** | Кнопка «Increment» — вызывается инструкция `increment` |
| **Обновить значение** | Кнопка «Refresh» — перечитывает счётчик из аккаунта |
| **Смотреть ошибки** | Блок «Ошибка» под кнопками |
| **Смотреть последние транзакции** | Блок «Последние транзакции» (подпись и время) |

Логика в `App.tsx`: PDA считается как `[b"counter", authority]`, вызываются `initialize()` и `increment()` с нужными `accounts` — этого достаточно, чтобы ручками прогнать контракт после деплоя на localhost.

## Запуск

1. В корне Anchor-проекта (`counter/`): `anchor build` и при необходимости `./app/setup.sh` или `make app-setup` (скопирует IDL из `target/idl/` в `app`).
2. Локальный валидатор: в отдельном терминале `solana-test-validator`.
3. Деплой: `solana config set --url localhost`, `solana airdrop 10`, `anchor deploy`.
4. Из папки `app`: `npm install` (один раз), `npm run dev`.
5. В Phantom: RPC = `http://127.0.0.1:8899`, затем открыть http://localhost:5173 и подключить кошелёк.

## Скрипты

- `npm run dev` — dev-сервер
- `npm run build` — сборка
- `npm run setup` — скопировать IDL из `../target/idl/counter.json`
