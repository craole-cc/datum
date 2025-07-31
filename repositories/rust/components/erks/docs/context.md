# Erks::Context

Wraps any lower-level error with a custom message.

| Category | Severity |
| -------- | -------- |
| Generic  | High     |

## When It Occurs

- You explicitly call `.context("…")` on a `Result`.
- You want to add human-friendly detail to an existing error.

## How to Fix

- Review the wrapped source error (if present) for root cause.
- Adjust your context message to clarify what operation failed.
- Use `.context` sparingly for high-level operations only.

## Example

```rust
fs::read_to_string(path)
    .context(format!("couldn’t load config at {:?}", path))?;
```
