# Defaults.
.PHONY: run
run: clippy_run

.PHONY: build
build: clippy_build

# Miri
.PHONY: miri_run
miri_run:
	MIRIFLAGS="-Zmiri-tag-raw-pointers" cargo miri run

.PHONY: miri_test
miri_test: 
	MIRIFLAGS="-Zmiri-tag-raw-pointers" cargo miri test

# Clippy
.PHONY: clippy_run
clippy_run: 
	cargo clippy && cargo run

.PHONY: clippy_build
clippy_build:
	cargo clippy && cargo build

.PHONY: clippy
clippy: 
	cargo clippy