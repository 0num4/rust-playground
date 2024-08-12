build:
	cargo build --release
preinstall:
	brew install postgresql
	brew install libpq
	cargo clean