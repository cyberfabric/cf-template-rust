# CyberFabric Rust Templates

A collection of `cargo-generate` templates for creating CyberFabric projects and modules.

## Templates

### 🚀 Init - Project Initialization

Creates a new CyberFabric project with workspace setup and example modules.

**Usage:**
```bash
cargo generate --git https://github.com/cyberfabric/cf-template-rust Init
```

**Includes:**
- Workspace configuration
- Example modules (hello-world, ipinfo-fetcher)
- cf-modkit integration
- Tokio async runtime setup

### 📦 Modules - Add New Modules

Interactive template for adding new modules to an existing CyberFabric project.

**Usage:**
```bash
cd your-project/modules
cargo generate --git https://github.com/cyberfabric/cf-template-rust Modules
```

**Features:**
- Interactive TUI prompts
- Multiple module types:
  - `simple-periodic`: Basic periodic tasks
  - `http-fetcher`: HTTP client with clean architecture
  - `custom`: Minimal template for custom logic
- Automatic README generation
- Conditional file inclusion based on module type

## Quick Start

1. **Create a new project:**
   ```bash
   cargo generate --git https://github.com/cyberfabric/cf-template-rust Init
   cd your-project-name
   cargo build
   ```

2. **Add a new module:**
   ```bash
   cd modules
   cargo generate --git https://github.com/cyberfabric/cf-template-rust Modules
   ```

3. **Update workspace** (add your new module to `Cargo.toml`):
   ```toml
   [workspace]
   members = ["modules/hello-world", "modules/your-new-module"]
   ```

4. **Build and run:**
   ```bash
   cargo build
   ```

## Requirements

- Rust toolchain (1.92.0 or later, edition 2024)
- `cargo-generate`: `cargo install cargo-generate`

## Documentation

- [Init Template Documentation](./Init/README.md)
- [Modules Template Documentation](./Modules/README.md)

## License

See [LICENSE](./LICENSE) and [NOTICE](./NOTICE) files.
