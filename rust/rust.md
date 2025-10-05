
# 0. 

## 0.1 Установка rustfmt


```sh
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh  
```
Cargo's bin directory ($HOME/.cargo/bin).

To configure your current shell, you need to source
the corresponding env file under $HOME/.cargo.

This is usually done by running one of the following (note the leading DOT):
. "$HOME/.cargo/env"            # For sh/bash/zsh/ash/dash/pdksh
source "$HOME/.cargo/env.fish"  # For fish
source $"($nu.home-path)/.cargo/env.nu"  # For nushell

```sh
rustc --version  
```




## 0.2 Настройка Rust-проекта с Cargo
https://practicum.yandex.ru/learn/middle-rust-blockchain/courses/0c7c693a-6212-46be-90a7-4a9f947ad137/sprints/736337/topics/a86d4845-32c7-4906-b112-d25dd120dfc1/lessons/c46a6356-4853-4fc9-961b-fe70984d7a0d/
```sh
cargo --list 
cargo new yandex_course 
cargo new yandex_course --lib # Создать проект библиотеки 
```

```sh 
 sudo snap install rustup --classic 
 rustup default stable
 ```


```sh 

 rustup component add rustfmt
 cargo fmt # запуск форматирования
 
 rustup component add clippy

```



## 0.3 Работа с модулями и зависимостями

```sh

cargo add time
cargo search 
```


Ссылка на урок: [Тема 2/5: Проекты и синтаксис Rust → Урок 4/9: Типы данных, переменные и функции в Rust]( https://practicum.yandex.ru/learn/middle-rust-blockchain/courses/0c7c693a-6212-46be-90a7-4a9f947ad137/sprints/736337/topics/a86d4845-32c7-4906-b112-d25dd120dfc1/lessons/8b50eeb3-e07b-4294-92c4-5ea8f39a8421/#e523b3cd-6cf1-4086-97e2-e6a0d2fa3151 )


## 0.4 Потоки 
Ссылка на урок: [Тема 2/5: Проекты и синтаксис Rust → Урок 5/9: Управление потоком: условия, циклы и паттерн-матчинг]( https://practicum.yandex.ru/learn/middle-rust-blockchain/courses/0c7c693a-6212-46be-90a7-4a9f947ad137/sprints/736337/topics/a86d4845-32c7-4906-b112-d25dd120dfc1/lessons/ef4f7679-3b2a-400d-9956-01af9fd876a5/#59397a48-65a9-4e98-9787-6c0666e4ec33 )
Вопрос: 