default:
	cd program && cargo prove build
	cd script && RUST_LOG=debug RUST_BACKTRACE=full RUSTCFLAGS="-C opt-level=3 -C debug-assertions" FRI_QUERIES=1 cargo run --release