test:
	cargo test --lib

coverage:
	cargo llvm-cov --lib --html

eval:
	RUST_BACKTRACE=1 cargo run --release --bin eval

profile-time-osx:
	cargo instruments -t time --release --bin main

