#!/bin/bash

set -ex;

CUR_DIR=$(dirname $0)

pushd $CUR_DIR ;
    cargo fmt;
    cargo clippy ;
    # cargo clippy --fix --bin "yandex_course";
    cargo build ;
    cargo check ;
    # cargo run ;
    # cargo run --example cli  
    cargo run --example cli -- balance Alice ;
    cargo run --example cli -- add Alice 5000 ;
    cargo run --example cli -- balance Alice  ;
    cargo run --example cli -- balance Alice  ;
    cargo test  ;
    # cargo run --bin utils
popd;