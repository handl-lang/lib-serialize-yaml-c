# lib-serialize-yaml-c

`handl-plugin-lib-serialize-yaml-c` — native YAML serializer backend for
lib-serialize (the `serde_yaml` crate over the serialize traits).

## Development

```bash
cargo xtask gate        # fmt --check → clippy -D warnings → test → doc
```

`cargo xtask` wraps the repo's `xtask/` crate (`fmt`, `clippy`, `test`, `doc`,
`check`, `gate`). Deps resolve from git, so no sibling checkouts are needed;
a plain clone test-runs as-is.

## License

MIT.
