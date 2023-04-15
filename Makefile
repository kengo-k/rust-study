build:
	rustc --out-dir dist/ src/main.rs

test:
	cargo test -- --nocapture