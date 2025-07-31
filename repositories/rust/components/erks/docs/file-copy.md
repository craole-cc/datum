# Erks::FileCopy

Failure copying a file from one path to another.

| Category   | Severity |
| ---------- | -------- |
| Filesystem | Medium   |

## Common Causes

- Source unreadable or missing.
- Destination directory missing or unwritable.
- Insufficient disk space at target.
- Cross-device copy errors (rename fallback).

## How to Fix

- Confirm source exists and is readable.
- Create destination directory and grant write perms.
- Check available space on target volume.
- If rename fails, try `fs::copy` + remove manually.

## Example

```rust
std::fs::copy("source.txt", "backup/source.txt")
    .context("failed to copy file to backup")?;
```
