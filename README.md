# ratatui-recipe

A small collection of utilities, patterns, and a cargo-generate template for building asynchronous terminal UIs with ratatui.

This repository provides a lightweight app framework and a reference template to bootstrap apps quickly. The design is inspired by and borrows ideas from [ratapp](https://github.com/Nekidev/ratapp) (it is essentially a 1:1 clone in its early state).

---

## Template (quick start)

A ready-to-use cargo-generate template is available in the template/ directory. The template produces a minimal application with a pages-based structure so you can start coding UI screens immediately.

What the template includes:

- `src/main.rs` — application bootstrap and global state
- `src/pages/mod.rs` — `Pages` enum and default page
- `src/pages/home.rs` — a tiny "Hello, world!" `StatefulPage`

Generate a new project from the repository:

```bash
cargo generate --path https://github.com/ls3205/ratatui-recipe.git
```

---

## After generation

1. Inspect `src/` in the generated project and add runtime dependencies to `Cargo.toml` (for example `tokio`, `ratatui`, and `ratatui-recipe`).
2. Build and run the app with `cargo run`.

Refer to the example files under `template/src` for implementation details and to the `passiogo_tui` example for a fuller app structure.

---

## Credits

Inspired by [ratapp](https://github.com/Nekidev/ratapp).
