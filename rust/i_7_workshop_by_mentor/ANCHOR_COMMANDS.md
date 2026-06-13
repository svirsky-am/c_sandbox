# Anchor — команды под рукой

Из папки `counter/` (корень Anchor-проекта).

## Сборка и тесты

| Команда | Описание |
|--------|----------|
| `anchor build` | Собрать программу, сгенерировать IDL и типы в `target/` |
| `anchor test` | Запустить валидатор, задеплоить программу, выполнить тесты из `tests/` |
| `anchor test --skip-build` | Тесты без пересборки (если уже делали `anchor build`) |

## Деплой

| Команда | Описание |
|--------|----------|
| `anchor deploy` | Деплой на текущий кластер (из `solana config get`; обычно localhost или devnet) |
| `anchor deploy --provider.cluster devnet` | Деплой на Devnet |
| `anchor deploy --provider.cluster mainnet` | Деплой на Mainnet (осторожно) |

Перед деплоем на localhost: запустить `solana-test-validator` в отдельном терминале и выполнить `solana config set --url localhost` и при необходимости `solana airdrop 10`.

## Ключи и конфиг

| Команда | Описание |
|--------|----------|
| `anchor keys list` | Показать Program ID (читает `target/deploy/*-keypair.json`; Makefile копирует туда `dev-counter-keypair.json` перед build/deploy) |
| `anchor keys sync` | Обновить Program ID в `Anchor.toml` и в коде из ключей в `target/deploy/` |

## Управление проектом

| Команда | Описание |
|--------|----------|
| `anchor init <имя>` | Создать новый Anchor-проект (не в уже существующем) |
| `anchor upgrade <program.so> --program-id <ID>` | Обновить уже задеплоенную программу |

## Через npm/yarn (из папки counter)

```bash
yarn build          # anchor build
yarn test           # anchor test
yarn deploy         # anchor deploy
yarn deploy:devnet  # anchor deploy --provider.cluster devnet
yarn keys           # anchor keys list
```

## Связка: Solana CLI

```bash
solana config set --url localhost   # или devnet / mainnet-beta
solana config get
solana airdrop 10

solana-test-validator               # локальный валидатор (отдельный терминал)
```
