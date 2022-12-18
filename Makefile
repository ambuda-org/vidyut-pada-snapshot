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
				   --hash "bf906cec17e58e84da96a30d38131f29a004b444eac6581e3679966dafec2482"

sanskrit_verb_eval:
	cargo run --release --bin sanskrit_verb_eval | tee sanskrit-verb-results.txt


# Profiles the program's execution time on OSX. This command will probably not
# work on other operating systems.
profile-time-osx:
	cargo instruments -t time --release --bin main

