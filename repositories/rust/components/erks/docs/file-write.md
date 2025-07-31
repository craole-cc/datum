# Erks::FileWrite

Failure writing data to a file (append, overwrite, or flush).

| Category   | Severity |
| ---------- | -------- |
| Filesystem | Medium   |

## Common Causes

- Disk is full.
- No write permission on the file.
- File was closed prematurely.
- Underlying FS corruption or hardware error.

## How to Fix

- Free up space or use a different volume.
- Change file permissions (`chmod +w`).
- Ensure the file handle is open and valid.
- Run FS checks if corruption is suspected.

## Example

```rust
use std::{fs::File, io::Write};

let mut file = File::create("log.txt")
    .context("failed to open log file")?;

file.write_all(b"Hello, world!")
    .context("failed to write to log file")?;
```
