.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: package
package:
	@projects=$$(find . -mindepth 1 -maxdepth 1 -type d | grep -Ev '(.git|target|testing)' | grep "$(PROJECT)"); \
	for project in $$projects; do \
		set -e ; \
		echo "Substreams packing $$project..."; \
		pushd $$project > /dev/null; \
		substreams build; \
		popd > /dev/null; \
	done

.PHONY: format
format:
	cargo fmt
