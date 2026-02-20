# CyberFabric Template Testing Makefile

.PHONY: help test test-fast test-full validate clean clean-all install-tools check-deps ci dev

# Default target
help:
	@echo "CyberFabric Template Testing"
	@echo "============================="
	@echo ""
	@echo "Available targets:"
	@echo "  make test          - Run quick validation tests"
	@echo "  make test-fast     - Alias for 'make test'"
	@echo "  make test-full     - Run full integration tests"
	@echo "  make validate      - Alias for 'make test'"
	@echo "  make install-tools - Install cargo-generate"
	@echo "  make check-deps    - Check if required tools are installed"
	@echo "  make clean         - Clean up test directories"
	@echo "  make clean-all     - Clean up everything including generated files"
	@echo ""
	@echo "Examples:"
	@echo "  make test          # Quick validation"
	@echo "  make test-full     # Full test suite"
	@echo "  make install-tools && make test-full"

# Quick validation (no cargo-generate required)
test:
	@echo "🧪 Running validation tests..."
	@cd tests && cargo test --lib
	@echo ""
	@echo "✅ Validation tests complete!"
	@echo ""
	@echo "To run full integration tests:"
	@echo "  make test-full"

test-fast: test

# Validate template structure and syntax
validate: test

# Full integration tests
test-full:
	@echo "🧪 Running full integration test suite..."
	@cd tests && cargo test -- --include-ignored
	@echo ""
	@echo "✅ All tests passed!"

# Check if required dependencies are installed
check-deps:
	@echo "🔍 Checking dependencies..."
	@command -v cargo >/dev/null 2>&1 || { echo "❌ cargo not found. Install Rust from https://rustup.rs/"; exit 1; }
	@echo "✅ All dependencies installed"

# Clean up test directories
clean:
	@echo "🧹 Cleaning up test directories..."
	@rm -rf /tmp/test-cf-*
	@rm -rf /tmp/test-simple-*
	@rm -rf /tmp/test-fetcher-*
	@rm -rf /tmp/test-custom-*
	@rm -rf /tmp/test-workspace
	@rm -rf /tmp/placeholder-test
	@rm -rf /tmp/compile-test
	@rm -rf /tmp/cf-*
	@cd tests && cargo clean 2>/dev/null || true
	@echo "✅ Cleanup complete"

# Clean up everything including generated documentation
clean-all: clean
	@echo "🧹 Removing test result files..."
	@rm -f TEST_RESULTS.md
	@rm -f TEST_SUMMARY.md
	@echo "✅ Full cleanup complete"

# CI/CD target
ci: test-full
	@echo "✅ All CI checks passed!"

# Development workflow
dev: test
	@echo "✅ Development validation passed"
	@echo ""
	@echo "Template is ready for development!"
