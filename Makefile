.PHONY: dev build dev-setup

# run this command to setup the dev clis needed in your system
dev-setup:
	cargo install cargo-watch systemfd

# Run development server
dev:
	systemfd --no-pid -s http::3000 -- cargo watch -c -q -x run

# Build for production
build:
	cargo build
