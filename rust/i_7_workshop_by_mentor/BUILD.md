# Сборка Counter (Anchor 0.32+)

## Последний Anchor (0.32.1)

Чтобы собирать проект на **последнем Anchor** (0.32.x), нужны два условия.

### 1. Agave (Solana CLI) — канал edge

Зависимость `constant_time_eq` 0.4.x требует Cargo с поддержкой **edition2024** (Rust 1.85+).  
В stable-канале Agave встроенный Cargo 1.84, поэтому сборка падает с ошибкой `edition2024 is required`.

**Решение:** переключить установку Solana/Agave на канал **edge** (там обновлённые platform-tools):

```bash
agave-install init edge
```

После этого `active_release` будет указывать на edge, и `anchor build` сможет собрать проект.

### 2. Anchor CLI 0.32.1

```bash
avm use latest
# или явно: avm install 0.32.1 && avm use 0.32.1
```

### Сборка

```bash
cd counter
anchor build
```

### Ключик программы (keypair)

В корне `counter/` лежит **`dev-counter-keypair.json`** — это ключик программы для удобства воркшопа. При `make build` и `make deploy-localnet` он автоматически копируется в `target/deploy/counter-keypair.json` (Anchor ожидает ключик там). У всех участников один и тот же program ID — IDL и фронт менять не нужно. На shared devnet один ключ = один адрес программы; для воркшопа обычно используют только localnet.