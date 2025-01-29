.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: package
package: build
	@projects=$$(find . -mindepth 1 -maxdepth 1 -type d | grep -v target | grep -v .git); \
	for project in $$projects; do \
		echo "Substreams packing $$project..."; \
		pushd $$project > /dev/null; \
		popd > /dev/null; \
	done
