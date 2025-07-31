# Erks::PathNotFound

The specified path could not be found.

| Category   | Severity |
| ---------- | -------- |
| Filesystem | Medium   |

## Common Causes

- Typo in the path.
- File or directory was moved/deleted.
- Relative path based on wrong working directory.

## How to Fix

- Correct the path spelling.
- Ensure your app’s working directory is what you expect.
- Use absolute paths if needed.

## Example

```rust
std::fs::read_to_string("missing.txt")
    .context("file not found")?;
```
