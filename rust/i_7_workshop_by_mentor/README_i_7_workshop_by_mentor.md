# Solana Dive — воркшоп #5

Воркшоп Solana: CLI, минтинг токенов, деплой смарт-контракта Counter на Anchor (localnet/devnet).

---

## Где что лежит

| Путь | Назначение |
|------|------------|
| **`scenario.md`** | Сценарий: подсказки по командам, тайминг, приложения. |
| **`description.md`** | Описание для автора/наставника, требования, рекомендации. |
| **`table.md`** | Таблицы/сводки по воркшопу. |
| **`counter/`** | Anchor-проект Counter (программа + тесты + фронт). |

### Внутри `counter/`

| Путь | Назначение |
|------|------------|
| **`Anchor.toml`** | Конфиг Anchor (сеть, program ID, скрипты тестов). |
| **`Makefile`** | Команды: `build`, `test`, `deploy-localnet`, `app-setup`, `app-dev`, `keys`, `airdrop-localnet`. |
| **`BUILD.md`** | Сборка под Anchor 0.32 (Agave edge, avm). |
| **`ANCHOR_COMMANDS.md`** | Шпаргалка по командам Anchor и Solana CLI. |
| **`programs/counter/`** | Rust-программа (счётчик, PDA, initialize/increment). |
| **`tests/`** | Интеграционные тесты (TypeScript). |
| **`app/`** | Фронт (React + Vite): подключение кошелька, Initialize/Increment, отображение сети. |
| **`app/setup.sh`** | Копирует IDL из `target/idl/` в `app/src/idl/` (запускать после `anchor build`). |
| **`dev-counter-keypair.json`** | Ключик программы (в корне `counter/`). Makefile подставляет его в `target/deploy/` перед сборкой и деплоем. |

---

## Ключик программы (dev key)

В репозитории закоммичен **`counter/dev-counter-keypair.json`** — это **разработческий** ключ программы, чтобы у всех студентов был один и тот же program ID и не нужно было менять IDL/фронт. **В продакшене ключи программ в репозиторий коммитить не рекомендуется** — храните их в секретах/менеджерах (env, vault и т.п.) и не пушите в git.

---

## Быстрый старт (localnet)

Из корня репозитория:

```bash
cd counter
make build
make app-setup
```

В **отдельном терминале** запустить валидатор:

```bash
solana-test-validator
```

В первом терминале:

```bash
make airdrop-localnet   # при необходимости
make deploy-localnet
make app-dev
```

В браузере: http://localhost:5173. В Phantom выставить RPC: `http://127.0.0.1:8899`, подключить кошелёк, нажать Initialize → Increment.

---

## Требования

- Rust (stable), Solana CLI (Agave), Anchor CLI (avm), Node.js (LTS)
- Для Anchor 0.32: Agave **edge** и `avm use latest` — см. **`counter/BUILD.md`**
- Кошелёк Phantom (или аналог) для фронта

---

## Полезные ссылки

- **Сборка и проблемы** — `counter/BUILD.md`
- **Команды Anchor** — `counter/ANCHOR_COMMANDS.md`
- **Фронт (запуск, тесты)** — `counter/app/README.md`


## Кратко последовательность команд
1. `anchor build`
2. `anchor keys list` - program id (declare_id!, и в anchor.toml)
3. `anchor build`
4. `anchor test`
5. `solana-test-validator`
6. `anchor-deploy`
7. `cd ./app`
8. `./setup.sh`
9. `yarn install` или `npm install`
10. `yarn dev` или `npm run dev`
