install:
	cargo build --release
	cp target/release/kurt /usr/local/bin/