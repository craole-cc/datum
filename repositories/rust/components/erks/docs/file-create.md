# Erks::FileCreate

Failure to create or truncate a file at the specified path.

| Category   | Severity |
| ---------- | -------- |
| Filesystem | Medium   |

## Common Causes

- Parent directory does not exist.
- Insufficient write permissions on the directory.
- Disk is full or quota exceeded.
- Path points to an existing directory, not a file.

## How to Fix

- Ensure parent directories exist (`mkdir -p`).
- Adjust directory permissions to allow file creation.
- Free up disk space or request larger quota.
- Confirm your target path isn’t a directory.

## Example

```rust
let file = std::fs::File::create("output.log")
    .context("failed to create output file")?;
```
