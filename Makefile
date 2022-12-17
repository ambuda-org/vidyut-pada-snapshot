# Generates all tinantas supported by the program.
create_tinantas:
	cargo run --release --bin generate

create_test_files:
	cargo run --release --bin create_test_file > test-files/tinanta.csv

# Runs all unit tests in the crate
test:
	cargo test --lib

# Generates a simple coverage report.
coverage:
	cargo llvm-cov --html

eval:
	RUST_BACKTRACE=1 cargo run --release --bin eval -- \
				   --test-cases test-files/tinanta.csv \
				   --hash "96719f11dc1c0b582c56d3bac40e4deed91ce7f0ef0b0dce29d7687063a06c63"

sanskrit_verb_eval:
	cargo run --release --bin sanskrit_verb_eval | tee sanskrit-verb-results.txt


# Profiles the program's execution time on OSX. This command will probably not
# work on other operating systems.
profile-time-osx:
	cargo instruments -t time --release --bin main

