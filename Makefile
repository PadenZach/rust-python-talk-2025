# Force uv to reinstall rust package locally
build:
	uv sync --reinstall

# Compare our baseline to rust.
benchmark:
	hyperfine "uv run python baseline.py" "uv run python rust_test.py" --warmup 1