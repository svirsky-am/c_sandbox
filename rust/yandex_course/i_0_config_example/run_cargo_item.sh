#!/bin/bash

set -ex;

CUR_DIR=$(dirname $0)

pushd $CUR_DIR ;
    cargo fmt;
    cargo clippy ;
    # cargo clippy --fix --bin "yandex_course";
    cargo build ;
    cargo check ;
    cargo run ;
popd;