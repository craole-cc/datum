# Erks::PathPermissionDenied

Operation failed due to insufficient permissions.

| Category   | Severity |
| ---------- | -------- |
| Filesystem | Medium   |

## Common Causes

- User lacks read/write/delete privileges.
- SELinux/AppArmor policies blocking access.

## How to Fix

- Adjust filesystem ACLs or Unix permissions.
- Update your security policy or context.

## Example

```rust
std::fs::remove_file("/etc/hosts")
    .context("permission denied while deleting system file")?;
```
