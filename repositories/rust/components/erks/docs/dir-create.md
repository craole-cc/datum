# Erks::DirCreate

Failure creating a new directory.

| Category   | Severity |
| ---------- | -------- |
| Filesystem | Medium   |

## Common Causes

- Parent directory does not exist.
- Insufficient permissions on parent.
- Path exists as a file.
- Filesystem is read-only.

## How to Fix

- Create parent directories first.
- Adjust parent permissions.
- Remove or rename the conflicting file.
- Mount filesystem with write access.

## Example

```rust
std::fs::create_dir_all("logs/2025/07")
    .context("failed to create log directory")?;
```
