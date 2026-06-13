# Anchor Counter — команды из корня counter/
# Использование: make build, make deploy-localnet и т.д.
# Ключик программы берётся из dev-counter-keypair.json (копируется в target/deploy перед build/deploy).

KEYPAIR_SOURCE := dev-counter-keypair.json
KEYPAIR_DEST := target/deploy/counter-keypair.json

.PHONY: build test deploy-localnet keys airdrop-localnet app-setup app-dev ensure-keypair

# Подставить ключик в target/deploy (Anchor ожидает его там)
ensure-keypair:
	@mkdir -p target/deploy
	@cp "$(KEYPAIR_SOURCE)" "$(KEYPAIR_DEST)"
	@echo "Keypair: $(KEYPAIR_SOURCE) -> $(KEYPAIR_DEST)"

# Сборка программы и IDL
build: ensure-keypair
	anchor build

# Тесты (с --skip-local-validator если валидатор уже запущен; иначе anchor поднимет сам)
test:
	anchor test --skip-local-validator

# Деплой в localnet (localhost).
# Перед этим в отдельном терминале запусти: solana-test-validator
deploy-localnet: ensure-keypair
	solana config set --url http://127.0.0.1:8899
	anchor deploy --provider.cluster localnet

# Выдать SOL на localnet (если баланса не хватает)
airdrop-localnet:
	solana config set --url http://127.0.0.1:8899
	solana airdrop 10

# Показать Program ID
keys:
	anchor keys list

# Скопировать IDL в app (после make build)
app-setup:
	./app/setup.sh

# Запустить dev-сервер фронта (localhost:5173)
app-dev:
	cd app && npm run dev
