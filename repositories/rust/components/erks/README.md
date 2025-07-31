# Erks - Enhanced Rust Error Handling

A comprehensive error handling library built on top of `miette` that provides
structured, diagnostic-rich errors with intelligent context mapping.

## Features

- **Structured Error Types**: Pre-defined error variants for common scenarios
  (file operations, path issues, etc.)
- **Intelligent Context Mapping**: Automatically maps `std::io::Error` to
  appropriate structured errors based on context
- **Rich Diagnostics**: Built-in severity levels, help messages, documentation
  URLs, and error codes
- **Proc Macro Support**: Automatic constructor generation and diagnostic
  implementation
- **Miette Integration**: Full compatibility with miette's fancy error reporting
  (optional)
- **Library-Friendly**: Works great with or without fancy features for library
  authors

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
erks = { path = "components/erks" }

# For applications that want fancy error reporting:
[dependencies]
erks = { path = "components/erks", features = ["fancy"] }
```

### Basic Usage

```rust
use erks::{Context, Error, Result};
use std::fs;

fn read_config(path: &str) -> Result<String> {
    // The context method will intelligently map io::Error to structured errors
    fs::read_to_string(path)
        .context(format!("Loading configuration from {}", path))
}

fn main() -> Result<()> {
    match read_config("/missing/config.toml") {
        Ok(content) => println!("Config: {}", content),
        Err(e) => {
            eprintln!("Error: {}", e);
            if let Some(help) = e.help() {
                eprintln!("Help: {}", help);
            }
        }
    }
    Ok(())
}
```

### Constructor Methods

```rust
use erks::Error;
use std::io;

// Direct constructor
let error = Error::file_read(
    io::Error::new(io::ErrorKind::NotFound, "not found"),
    "/path/to/file.txt"
);

// Constructor with context
let error = Error::file_read_with_context(
    io::Error::new(io::ErrorKind::PermissionDenied, "access denied"),
    "/root/secret.txt",
    "loading sensitive configuration"
);
```

## Error Variants

The library provides several built-in error variants each including:

- **Severity Level**: Low, Medium, High, Critical
- **Error Code**: Structured identifier (e.g., `erks::file_read`)
- **Help Message**: Actionable guidance for users
- **Documentation URL**: Links to detailed documentation
- **Source Chain**: Preserves underlying error information

| Variant                                            | Category   | Severity |
| -------------------------------------------------- | ---------- | -------- |
| [Context](docs/context.md)                         | Generic    | High     |
| [DirCreate](docs/dir-create.md)                    | Filesystem | Medium   |
| [DirDelete](docs/dir-delete.md)                    | Filesystem | Medium   |
| [DirRead](docs/dir-read.md)                        | Filesystem | Medium   |
| [FileCopy](docs/file-copy.md)                      | Filesystem | Medium   |
| [FileCreate](docs/file-create.md)                  | Filesystem | Medium   |
| [FileDelete](docs/file-delete.md)                  | Filesystem | Medium   |
| [FileMove](docs/file-move.md)                      | Filesystem | Medium   |
| [FileRead](docs/file-read.md)                      | Filesystem | Medium   |
| [FileWrite](docs/file-write.md)                    | Filesystem | Medium   |
| [PathAlreadyExists](docs/path-already-exists.md)   | Filesystem | Low      |
| [PathNotFound](docs/path-not-found.md)             | Filesystem | Medium   |
| [PermissionDenied](docs/path-permission-denied.md) | Filesystem | High     |

## Examples

### Running the Simple Demo (Library Usage)

For library authors or when you don't need fancy error reporting:

```bash
cargo run --example simple_demo
```

This example shows:

- Basic error handling without fancy features
- Three different ways to create errors (context trait, constructors, manual)
- How to display error information programmatically
- Library-friendly error handling patterns

### Running the Fancy Demo (Application Usage)

For applications that want beautiful error reports:

```bash
cargo run --example fancy_demo --features fancy
```

This example shows:

- Fancy error reporting with terminal colors and formatting
- Error chaining and context
- Integration with miette's reporting features

## Architecture

```rs
erks/
├── src/
│   ├── lib.rs          # Main exports and re-exports
│   ├── error.rs        # Error enum definition
│   ├── context.rs      # Context trait for ergonomic error handling
│   ├── result.rs       # Result type alias
│   └── severity.rs     # Domain-specific severity levels
├── macros/
│   └── src/lib.rs      # Proc macros for constructor generation
└── examples/
    ├── simple_demo.rs  # Library usage without fancy features
    └── fancy_demo.rs   # Application usage with fancy features
```

## Design Principles

1. **No External Dependencies for Users**: Everything needed is re-exported
   through erks
2. **Library-Friendly**: Works great without fancy features for library code
3. **Application-Ready**: Optional fancy features for end-user applications
4. **Intelligent Mapping**: Context messages are analyzed to create appropriate
   error types
5. **Rich Diagnostics**: Every error provides actionable information

## Testing

```bash
# Run all tests
cargo test

# Run integration tests
cargo test --test integration

# Test with fancy features
cargo test --features fancy
```

## License

[LICENSE-APACHE](../../../../LICENSE-APACHE) and
[LICENSE-MIT](../../../../LICENSE-MIT)
