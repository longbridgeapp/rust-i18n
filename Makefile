release\:macro:
	cd crates/macro && cargo release --no-dev-version --skip-tag --skip-push
release\:support:
	cd crates/support && cargo release --no-dev-version --skip-tag --skip-push
release\:extract:
	cd crates/extract && cargo release --no-dev-version --skip-tag --skip-push
release:
	cargo release