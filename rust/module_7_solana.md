
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


solana balance
solana-keygen pubkey

solana config set --url devnet

3t5rc2K47B1Jvzzwk1RDhychLBrYTMJMYoqyDmZKPEEV



solana transfer 3Ki2o9FQr7CD3jRUJ1xqQygiyQ81JKQaoJ47Pb2bkecR 0.1 --allow-unfunded-recipient
```
pubkey: 3t5rc2K47B1Jvzzwk1RDhychLBrYTMJMYoqyDmZKPEEV
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


# localnet
```sh
solana config set --url localhost
solana airdrop 100
spl-token create-token

3t5rc2K47B1Jvzzwk1RDhychLBrYTMJMYoqyDmZKPEEV

```sh

#svirsky@svirsky-home-client:~/repos/c_sandbox$
 spl-token create-token
Creating token BX54f9LBVmtgEWYcKDafdPnhqmXH9q7KkJW4TZm3xjwk under program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA

Address:  BX54f9LBVmtgEWYcKDafdPnhqmXH9q7KkJW4TZm3xjwk
Decimals:  9

Signature: 5tJSg9jbajxiWb9yR7btfNJCj8TabF4sGBqqK2biDfWixci9rwcj52wSuTSAZB4EaJ4ZMmDfwts2P7e3EwgJpSk2

```
```sh
    #spl-token create-account --program-id <ADDRESS> <TOKEN_MINT_ADDRESS>

#     Creating token FTFV7fFezNzJAEsS3ukEDo78XtuGeLF446NhmoT74M8R under program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA

# Address:  FTFV7fFezNzJAEsS3ukEDo78XtuGeLF446NhmoT74M8R
# Decimals:  9
spl-token create-account --program-id TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA 3t5rc2K47B1Jvzzwk1RDhychLBrYTMJMYoqyDmZKPEEV





spl-token create-account  3t5rc2K47B1Jvzzwk1RDhychLBrYTMJMYoqyDmZKPEEV
    solana-keygen pubkey

spl-token accounts

```

# QA
- что будет, если к валидатору придут две конфликтующие транзакии с одним poh?
- 