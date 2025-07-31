# Erks::PathAlreadyExists

Path already exists when creating a file or directory.

| Category   | Severity |
| ---------- | -------- |
| Filesystem | Low      |

## Common Causes

- Attempt to create without checking for existing items.
- Race conditions in parallel workflows.

## How to Fix

- Check `.exists()` before creating.
- Use `OpenOptions::create(true).write(true)` to truncate safely.
- Remove or rename the existing item first.

## Example

```rust
use std::{fs::{File, remove_file,} path::Path};

if Path::new("output.txt").exists() {
  remove_file("output.txt")
    .context("failed to remove existing output file")?;
}

File::create("output.txt")
  .context("failed to create output file")?;
```
