# Template Tests

This crate contains Rust-based tests for the CyberFabric template repository.

## Structure

- `src/lib.rs` - Common test utilities
- `src/validation.rs` - Template structure validation tests
- `src/generation.rs` - Template generation and module tests
- `tests/integration.rs` - Integration tests

## Running Tests

### Quick validation tests (no cargo-generate required)
```bash
cargo test --lib
```

Or from the repository root:
```bash
make test
```

### Full integration tests (requires cargo-generate)
```bash
cargo test
```

Or from the repository root:
```bash
make test-full
```

### Run specific tests
```bash
# Run only validation tests
cargo test --lib

# Run only integration tests
cargo test --test integration

# Run ignored tests (requires cargo-generate)
cargo test -- --ignored
```

## Test Categories

### Validation Tests (Always Run)
- `test_init_template_structure` - Validates Init template files exist
- `test_modules_template_structure` - Validates Modules template files exist
- `test_placeholders` - Checks required placeholders are present
- `test_conditional_syntax` - Validates Liquid template syntax

### Generation Tests (Ignored by default, requires cargo-generate)
- `test_generate_simple_periodic` - Tests simple-periodic module generation
- `test_generate_http_fetcher` - Tests http-fetcher module generation
- `test_generate_custom` - Tests custom module generation

## Dependency Management

This crate also serves as the centralized location for dependency version management.

The `[workspace.metadata.test-dependencies]` section in `Cargo.toml` defines the versions used by:
- Makefile test targets
- test-all-modules.sh script
- Generated test workspaces

To update dependency versions:
1. Update versions in `tests/Cargo.toml` under `[workspace.metadata.test-dependencies]`
2. Update the corresponding versions in:
   - `Makefile` (test-simple, test-fetcher, test-custom targets)
   - `test-all-modules.sh` (workspace dependency sections)

## Adding New Tests

1. Add test functions to `tests/integration.rs`
2. Use utilities from `src/lib.rs`, `src/validation.rs`, or `src/generation.rs`
3. Mark tests that require cargo-generate with `#[ignore]`

Example:
```rust
#[test]
fn test_my_validation() {
    let template_dir = template_dir();
    // Your test logic here
}

#[test]
#[ignore] // Requires cargo-generate
fn test_my_generation() {
    // Your generation test logic here
}
```
