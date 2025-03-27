.PHONY:fmt
fmt:
	cargo fmt
	find . -name Cargo.toml -execdir cargo tomlfmt \;

.PHONY:sweep
sweep:
	cargo sweep --installed
