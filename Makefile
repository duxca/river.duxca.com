.PHONY:fmt
fmt:
	cargo fmt
	find . -name Cargo.toml -execdir cargo tomlfmt \;
	terraform -chdir=terraform fmt -recursive

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
	cargo clippy -- -D warnings
	cargo fmt --all -- --check
	terraform -chdir=terraform fmt -recursive -check

.PHONY:all
all:
	cargo build --workspace
	cd browser && trunk build --release
