# Erks::FileRead

Failure when trying to read from a file into memory or buffer.

| Category   | Severity |
| ---------- | -------- |
| Filesystem | Medium   |

## Common Causes

- File does not exist at the given path.
- Insufficient read permissions.
- File is locked by another process.
- Underlying I/O device error (disk failure, network FS issues).

## How to Fix

- Verify the file path is correct and the file exists.
- Check user permissions and adjust with `chmod`/ACLs.
- Ensure no other program holds an exclusive lock.
- Inspect lower‐level I/O error for device specifics.

## Example

```rust
let contents = std::fs::read_to_string("config.toml")
    .context("failed to read config file")?;
```
