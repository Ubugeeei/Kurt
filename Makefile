install:
	cargo build --release
	cp target/release/kurt /usr/local/bin/

sample:
	lsof -i:3000 -t | xargs kill
	cd example/server && cargo run &
	cargo run

exit:
	lsof -i:3000 -t | xargs kill
