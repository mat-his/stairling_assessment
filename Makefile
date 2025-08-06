.PHONY: install test run run-web run-db run-service1 run-service2 clean format lint

# Install dependencies for all services
install:
	cargo build --workspace

# Run tests for all services
test:
	cargo test --workspace

# Run the web service
run:
	cd web && cargo run

format:
	cargo fmt

lint:
	cargo clippy --workspace

doc:
	cat README.md

# Clean all services
clean:
	cargo clean --workspace
