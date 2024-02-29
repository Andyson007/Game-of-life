target/relaese/lifegif: src/main.rs
	cargo b --release
	time ./target/release/lifegif
