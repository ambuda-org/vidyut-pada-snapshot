test:
	cargo test --lib

coverage:
	cargo llvm-cov --lib --html

profile-time-osx:
	cargo instruments -t time --release

