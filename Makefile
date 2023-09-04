release\:macro:
	cd crates/macro && cargo release --no-dev-version --skip-tag --skip-push
release\:support:
	cd crates/support && cargo release --no-dev-version --skip-tag --skip-push
release\:extract:
	cd crates/extract && cargo release --no-dev-version --skip-tag --skip-push
release:
	cargo release
test:
	cargo test --workspace
	cargo test --manifest-path examples/app-workspace/Cargo.toml --workspace
	cargo test --manifest-path examples/share-locales-in-workspace/Cargo.toml --workspace
