# GitHub emoji for Rust

Full, up-to-date database of [GitHub emoji](https://github.com/github/gemoji) which have Unicode equivalents. Hashed at compile time for fast lookup.

## Example usage

```rust
let emoji = gh_emoji::get("smile");
assert_eq!(emoji, Some("😄"));
```

```rust
let replacer = gh_emoji::Replacer::new();
let text = replacer.replace_all(":crocodile:, see you in a while!");
```
