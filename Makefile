# Main commands
# ~~~~~~~~~~~~~

# Generates all tinantas supported by the program and writes them to stdout.
create_tinantas:
	cargo run --release --bin generate


# Unit tests
# ~~~~~~~~~~

# Runs all unit tests in the crate.
integration_:
	cargo test --lib

# Generates a simple coverage report and writes it to disk as an HTML file.
coverage:
	cargo llvm-cov --html


# Integration tests
# ~~~~~~~~~~~~~~~~~

# Generates all tinantas supported by the program and writes them to disk.
create_test_files:
	cargo run --release --bin create_test_file > test-files/tinanta.csv

# Runs a full evaluation over all forms generated by vidyut-prakriya. `hash` is
# the SHA-256 hash of the test file. We use `hash` to verify test file
# integrity and ensure that our test cases are stable.
test_all:
	RUST_BACKTRACE=1 cargo run --release --bin test_tinantas -- \
				   --test-cases test-files/tinanta.csv \
				   --hash "adbbfc7975741f6938f8498324d2da15eb01593f062c70973fb1b963f0fdf45f"

# Work-in-progress test function for subantas.
test_subantas:
	RUST_BACKTRACE=1 cargo run --release --bin test_subantas

# Performance
# ~~~~~~~~~~~

# Profiles the program's execution time on OSX. This command will probably not
# work on other operating systems.
profile-time-osx:
	cargo instruments -t time --release --bin generate


# Other
# ~~~~~

# Generates project docs and opens them in your default browser.
docs:
	cargo doc --no-deps --open
