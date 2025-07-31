# Erks::FileMove Error

## Description

Failure moving (renaming) a file from one location to another.

| Category   | Severity |
| ---------- | -------- |
| Filesystem | Medium   |

## Common Causes

- Cross-filesystem moves aren’t supported by `rename`.
- No write permission on destination directory.
- File is open/locked by another process.
- Source path doesn’t exist.

## How to Fix

- For cross-FS, fallback to copy+delete.
- Ensure you can write to the destination directory.
- Close any open handles before renaming.
- Verify source path is correct.

## Example

```rust
std::fs::rename("draft.txt", "archive/draft.txt")
    .context("failed to move draft to archive")?;
```
