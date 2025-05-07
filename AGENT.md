# Agent Guidelines for jj-lsp

## Build & Test Commands
- Build: `cargo build`
- Run: `cargo run`
- Test all: `cargo test`
- Test single: `cargo test test_name` (e.g., `cargo test test_diff_two_sides`)  
- Test with snapshot updates: `cargo test -- --update-snapshots`
- Install: `cargo install --path .`

## Code Style Guidelines
- **Imports**: Group and order by std, external crates, then internal modules
- **Error Handling**: Use `Option<T>` for recoverable absences and early returns
- **Naming**: Use snake_case for variables/functions, CamelCase for types/structs
- **Types**: Use explicit types for function signatures, prefer ownership over borrowing when appropriate
- **Formatting**: Follow Rust standard formatting (rustfmt)
- **Tests**: Use snapshot testing with insta crate for complex test assertions
- **Regex**: Define regex patterns with lazy_static
- **Modules**: Keep related functionality in dedicated modules

## Project Structure
- `src/main.rs`: Entry point for the LSP server
- `src/backend.rs`: LSP backend implementation
- `src/conflict.rs`: Conflict parsing and analysis
- `src/types.rs`: Shared type definitions
- `src/utils.rs`: Utility functions