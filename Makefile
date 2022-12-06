# Runs all unit tests in the crate
test:
	cargo test --lib

# Generates a simple coverage report.
coverage:
	cargo llvm-cov --lib --html

eval:
	RUST_BACKTRACE=1 cargo run --release --bin eval

# Profiles the program's execution time on OSX. This command will probably not
# work on other operating systems.
profile-time-osx:
	cargo instruments -t time --release --bin main

