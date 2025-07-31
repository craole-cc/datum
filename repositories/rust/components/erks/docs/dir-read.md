# Erks::DirRead

Failure reading entries of a directory.

| Category   | Severity |
| ---------- | -------- |
| Filesystem | Medium   |

## Common Causes

- Directory does not exist.
- No read or execute permission on the directory.
- Filesystem error.

## How to Fix

- Verify directory exists.
- `chmod +rX` to grant read+traverse.
- Run FS integrity checks if needed.

## Example

```rust
for entry in std::fs::read_dir("logs")
    .context("failed to read logs directory")? {
    let entry = entry.context("failed to read directory entry")?;
    println!("{:?}", entry.path());
}
```
