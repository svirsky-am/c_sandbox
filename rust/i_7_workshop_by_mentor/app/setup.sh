#!/usr/bin/env bash
# =============================================================
# setup.sh — настройка Dev-UI после сборки Anchor-программы
# =============================================================
# Запуск: ./setup.sh  (из папки app/)
# =============================================================

set -e

COUNTER_DIR="$(cd "$(dirname "$0")/.." && pwd)"
IDL_SOURCE="$COUNTER_DIR/target/idl/counter.json"
IDL_DEST="$(dirname "$0")/src/idl/counter.json"
APP_TSX="$(dirname "$0")/src/App.tsx"

echo "=== Counter Dev-UI Setup ==="
echo ""

# 1. Проверить что IDL существует
if [ ! -f "$IDL_SOURCE" ]; then
  echo "❌ IDL не найден: $IDL_SOURCE"
  echo ""
  echo "Сначала соберите программу:"
  echo "  cd .. && anchor build   # из папки counter/"
  exit 1
fi

# 2. Скопировать IDL
cp "$IDL_SOURCE" "$IDL_DEST"
echo "✅ IDL скопирован: $IDL_DEST"

# 3. Извлечь Program ID из IDL
PROGRAM_ID=$(python3 -c "import json,sys; d=json.load(open('$IDL_DEST')); print(d.get('address',''))" 2>/dev/null || echo "")

if [ -z "$PROGRAM_ID" ] || [ "$PROGRAM_ID" = "PLACEHOLDER_PROGRAM_ID_REPLACE_AFTER_BUILD" ]; then
  echo ""
  echo "⚠️  Program ID в IDL — placeholder."
  echo "   Запустите 'anchor keys list' и обновите:"
  echo "   1. declare_id!(\"...\") в programs/counter/src/lib.rs"
  echo "   2. counter = \"...\" в counter/Anchor.toml"
  echo "   3. Пересоберите: anchor build"
  echo "   4. Снова запустите ./setup.sh"
else
  echo "✅ Program ID: $PROGRAM_ID"
fi

echo ""
echo "==================================="
echo "Готово! Запустите Dev-UI:"
echo "  yarn install  (если первый раз)"
echo "  yarn dev"
echo "==================================="
