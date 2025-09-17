# Kms
Imma Kms

# Aidoku Multi-Source Extension

This project is a custom multi-source extension for the Aidoku manga reader app, written in Rust. It is designed to make it easy to add and manage multiple sources. The first source implemented is [mangabuddy.com](https://mangabuddy.com).

## Project Structure

- `src/lib.rs`: Main entry point, registers all sources.
- `sources/`: Folder containing individual source implementations (e.g., `mangabuddy.rs`).
- `Cargo.toml`: Rust project configuration.

## Adding a New Source

1. Create a new file in the `sources/` folder (e.g., `sources/newsource.rs`).
2. Implement the required Aidoku source logic in that file.
3. In `src/lib.rs`, add a `mod` statement and call the new source's `register()` function in `register_sources()`.

Example:
```rust
mod sources;
use sources::{mangabuddy, newsource};

#[no_mangle]
pub extern "C" fn register_sources() {
    mangabuddy::register();
    newsource::register();
}
```

## Building

To build the extension, run:
```bash
cargo build --release
```

## Contributing

See the original [aidoku-community-sources](https://github.com/Aidoku-Community/sources) repo for more info on source development.

---

This project is a starting point. The actual source logic for mangabuddy.com still needs to be implemented.
