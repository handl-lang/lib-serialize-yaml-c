# lib-serialize-yaml-c

`handl-plugin-lib-serialize-yaml-c` — native YAML serializer backend
for `lib-serialize` (the `serde_yaml` crate over the serialize
traits).

## Layout

| path | role |
| --- | --- |
| `src/lib.rs` | the Rust plugin crate (`handl-plugin-lib-serialize-yaml-c`) |
| `src/index.hnl` | the interface — `yaml_c.*` op declarations |
| `atoms.yaml`, `Cargo.toml`, `xtask/` | atoms / manifest / `cargo xtask` gate |
| `pkg.hnl`, `scripts/hnl-gate.sh`, `workflows/gate.yaml` | manifest (dep `serialize`) / hnl gate / bupha entry |

## Contract

- Transitional layout: interface is `src/index.hnl`; impl is the
  Rust crate — `yaml_c.serialize`/`deserialize` over `serde_yaml`
  via the `lib-serialize` traits.
- Deps resolve from git — a plain clone test-runs as-is.

## Verification

```bash
cargo xtask gate        # fmt --check → clippy -D warnings → test → doc
```

## License

MIT.

<!-- myco:agent-awareness -->

## Agent awareness

- Session start / any work discussion: run `myco status --prompt`
  before proposing next steps — know the board first.
- User describes new work → offer to capture it (`myco todo add`,
  `issue new`, `proposal new`). Don't wait to be asked.
- Picking up a task: `myco resume <ID>` is the briefing.
- Wrapping up: `myco handoff <ID>` anything resumable; keep statuses
  honest (claimed → in-progress → blocked/done).

<!-- /myco:agent-awareness -->
