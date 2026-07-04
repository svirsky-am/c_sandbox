# Debian/Ubuntu
sudo apt install build-essential cmake ninja-build python3 git pkg-config libssl-dev
# Кросс-компилятор и линкер для aarch64
sudo apt install gcc-aarch64-linux-gnu g++-aarch64-linux-gnu

```sh
# Базовые зависимости
sudo apt update
sudo apt install build-essential cmake ninja-build python3 git curl pkg-config libssl-dev

# Кросс-компилятор и линкер для aarch64
sudo apt install gcc-aarch64-linux-gnu g++-aarch64-linux-gnu

# Установка Rust (если еще не установлен)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustup default stable

# Клонируем репозиторий Rust
git clone https://github.com/rust-lang/rust.git
cd rust

# Инициализируем сабмодули. 
# Это скачает правильную версию LLVM в папку src/llvm-project
git submodule update --init --recursive
```
# Для кросс-компиляции понадобятся C-тулчейны для целевых платформ
# Например: sudo apt install gcc-aarch64-linux-gnu g++-aarch64-linux-gnu

https://github.com/rust-lang/rust/blob/1.77.2/.gitmodules
[bb7c483e48ea992f3759fb643962e1e706c8c029](https://github.com/rust-lang/rust/commit/bb7c483e48ea992f3759fb643962e1e706c8c029)