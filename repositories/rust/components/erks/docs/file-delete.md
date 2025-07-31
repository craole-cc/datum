# Erks::FileDelete

Failure deleting/removing a file from disk.

| Category   | Severity |
| ---------- | -------- |
| Filesystem | Medium   |

## Common Causes

- No delete (`unlink`) permission on the file.
- File is in use or locked by another process.
- Path doesn’t point to a file but to a directory.
- Network share disconnected or read-only.

## How to Fix

- Grant delete permissions.
- Close all handles—stop programs using the file.
- Use directory‐delete logic if it’s a folder.
- Remount network share with write access.

## Example

```rust
std::fs::remove_file("temp.txt")
    .context("failed to delete temporary file")?;
```
