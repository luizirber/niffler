all: check test test_benchmarks bench

check:
	cargo fmt
	cargo test
	cargo clippy

test:
	cargo test --all-features

test_benchmarks:
	cargo test --benches --all-features

bench:
	cargo bench --all-features

semver:
	cargo +nightly semver
# cargo +nightly semver -S niffler:1.0.0 -C niffler:2.0.0
