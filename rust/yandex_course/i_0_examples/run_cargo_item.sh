#!/bin/bash

set -ex;

pushd ./rust/yandex_course/i_0_examples/ ;
    cargo fmt;
    cargo clippy ;
    # cargo clippy --fix --bin "yandex_course";
    cargo build ;
    cargo check ;
    cargo run ;
popd;