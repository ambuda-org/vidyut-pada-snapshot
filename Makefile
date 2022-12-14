# Generates all tinantas supported by the program.
generate_tinantas:
	cargo run --release --bin generate

generate_test_file:
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
				   --hash "e10800020c5cc7af563546635a787eee0cef5462fd296cde1911fac4395b0e58"

# Profiles the program's execution time on OSX. This command will probably not
# work on other operating systems.
profile-time-osx:
	cargo instruments -t time --release --bin main

