test:
	@if [ -z $(n) ]; then \
		echo "Error: Function name needed when testing exactly that function!"; \
		echo "Example: make test n=hello_test"; \
		exit 1; \
	fi
	cargo test $(n) -- --exact --nocapture

test-all:
	cargo test