
# install
```sh
# solana cli
sh -c "$(curl -sSfL https://release.anza.xyz/v3.1.5/install)" 

# anchor
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
avm install 0.32.0
anchor --version


nvm install 20
nvm use 20
nvm alias default 20
npm install --global yarn
yarn --version
```




# using
```sh
export PATH="/home/svirsky/.local/share/solana/install/active_release/bin:$PATH"


solana --help 
solana-keygen new --no-bip39-passphrase --outfile rust/yandex_course/i_7_staff/wallet.json 

```
pubkey: 3Ki2o9FQr7CD3jRUJ1xqQygiyQ81JKQaoJ47Pb2bkecR
=========================================================================
Save this seed phrase to recover your new keypair:
confirm receive doll leader snack sign dad despair habit drop annual trip
=========================================================================

```sh
# Просмотр ключа
solana-keygen pubkey rust/yandex_course/i_7_staff/wallet.json
# проверка публичного ключа
solana-keygen verify 3Ki2o9FQr7CD3jRUJ1xqQygiyQ81JKQaoJ47Pb2bkecR rust/yandex_course/i_7_staff/wallet.json #  solana-keygen verify [OPTIONS] <PUBKEY> [KEYPAIR]
solana config get 



```

# QA
- что будет, если к валидатору придут две конфликтующие транзакии с одним poh?
- 