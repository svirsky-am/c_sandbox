.PHONY: run-test-of-libs
run-test-of-libs:
	cargo test -p rr-parser-lib

.PHONY: run-test-of-bin
run-test-of-bin:
	cargo test -p rr-file-processor

.PHONY: build-and-exec
build-and-exec:
	cargo build -p rr-file-processor
	cargo run -p rr-file-processor -- \
		--input rr-file-processor/tests/test_files/data.json \
		--output output/formatted/result.json

# .PHONY: build-and-exec
# build-and-exec:
# 	cargo build -p rr-file-processor
# 	cargo run -p rr-file-processor -- tests/test_files/hello.txt tests/test_files/rust.txt


.PHONY: linting
linting:
	cargo fmt
	cargo clippy
	cargo check