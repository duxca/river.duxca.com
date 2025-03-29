.PHONY:fmt
fmt:
	cargo fmt
	find . -name Cargo.toml -execdir cargo tomlfmt \;

.PHONY:sweep
sweep:
	cargo sweep --installed

.PHONY:clean
clean:
	cargo clean
	rm -rf db/target

.PHONY:watch
watch:
	cargo watch -x "clippy -p server"

.PHONY:check
check:
	cargo clippy
