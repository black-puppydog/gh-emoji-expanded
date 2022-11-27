# Expanded emoji set for Rust

Full, up-to-date database of emojis which have Unicode equivalents, taken from [GitHub emoji](https://github.com/github/gemoji) and [emoji-data](https://github.com/iamcal/emoji-data) which contains a few more entries.
If you spot noteworthy updates in either of these and don't see the changes reflected here, please open an issue. 🙏

This is a fork of [gh-emoji](https://github.com/kornelski/gh-emoji), which remains scoped to track exactly the short-codes from the `gemoji` package.

Pre-generated and hashed at compile time for fast lookup.

Useful when rendering [GitLab](https://gitlab.com/gitlab-org/gitlab-ce/blob/master/doc/user/markdown.md#emoji)/[GitHub-flavored  Markdown](https://github.github.com/gfm/), although this crate does not parse any Markdown itself.
I needed this for a re-implementation of [ssb-markdown](https://github.com/ssbc/ssb-markdown/) which uses [node-emoji](https://github.com/omnidan/node-emoji). However, it seems from cursory testing that `node-emoji` is also used in other popular apps like e.g. Slack, which means this might be more broadly useful.
If you find discrepancies between `node-emoji` and this crate, file an issue. 🙂

## Example usage

```rust
// yes, emoji-data has an extra robot_face entry! 🤖
assert_eq!(gh_emoji::get("robot"), gh_emoji::get("robot_face"));
```

```rust
let replacer = gh_emoji::Replacer::new();
let text = replacer.replace_all(":crocodile:, see you in a while!");
```
