# zed-orm-core — moved to `zed-lib-core`

> **Historical repository.** The `zed-orm-core` Rust crate is now maintained in
> [`zed-pkg/zed-lib-core`](https://github.com/zed-pkg/zed-lib-core) at
> [`src/rust-orm`](https://github.com/zed-pkg/zed-lib-core/tree/main/src/rust-orm).
> This repository remains available so existing commit pins, branches, issues,
> pull requests, and audit links continue to resolve.

The crate name and Rust import remain compatible:

```rust
use zed_orm_core::{connect_read_only, read};
```

Only the Git source changes.

## Consumer migration

Default read-only consumer:

```toml
zed-orm-core = {
  git = "https://github.com/zed-pkg/zed-lib-core.git",
  rev = "<reviewed-zed-lib-core-commit>"
}
```

API/write consumer:

```toml
zed-orm-core = {
  git = "https://github.com/zed-pkg/zed-lib-core.git",
  rev = "<reviewed-zed-lib-core-commit>",
  default-features = false,
  features = ["read-write"]
}
```

The package continues to live in `src/rust-orm` with package name
`zed-orm-core`. Default builds remain read-only; API servers enable
`read-write`; only the discrete migration job enables `migrate`.

## Preserved merge history

The canonical repository is a two-parent semantic merge of the `zed-lib` and
`zed-orm-core` histories:

```text
f27f72cc65640407409d38953c8d30ee4c95f3a6
```

Parents:

```text
430aafe24b6c3ab1263f1351ab4941545f592f19  zed-lib lineage
a5dabf3685db94ffdf5ae30cb3b3e4cc1cce298f  zed-orm-core lineage
```

The conceptual fold is:

```text
9fdc5fed96b707b99b3b02e6541060831c3d70fd
```

Canonical package, schema, API, storage, and feature-boundary certification
merged through [`zed-lib-core#1`](https://github.com/zed-pkg/zed-lib-core/pull/1)
as:

```text
171ee6a3ba82a492409ef86e27af793574942447
```

## Canonical contract

The merged crate preserves and extends the original boundary:

1. **Schema ownership is external.** The exact shared registry SQL comes from
   `ORESoftware/k8s-libs-and-shared-defs`, pinned by
   `zed-lib-core/shared-defs.lock.json`.
2. **Raw sessions do not escape.** Consumers receive opaque `ReadContext` or
   `WriteContext` values and call named `read`, `registry`, `write`, or
   feature-gated `invitations` operations.
3. **Writes remain opt-in.** Default builds cannot import write or migration
   symbols; the authoritative control remains the database principal.
4. **Canonical tables use the `zed_` prefix.** Transitional unprefixed tables
   and independently authored migration schemas are retired.
5. **Public errors are stable.** The crate exposes `OrmError`, not raw SeaORM or
   SQLx backend errors.

One-time invitation acceptance merged through
[`zed-lib-core#2`](https://github.com/zed-pkg/zed-lib-core/pull/2) as
`79c30f65c676f6eb304effe2a7abf969f22f2da8`.

The remaining registry upload/download/license/embedding and search operations
are tracked by [`zed-lib-core#3`](https://github.com/zed-pkg/zed-lib-core/pull/3).
The item-by-item predecessor mapping is in
[`PREDECESSOR_MIGRATION.md`](https://github.com/zed-pkg/zed-lib-core/blob/main/PREDECESSOR_MIGRATION.md).

## Repository policy

- Do not open new feature or release work here.
- Do not publish a new crate or repository-level release from this repository.
- Keep historical branches and commits available for audit.
- New bugs, features, and pull requests belong in
  [`zed-pkg/zed-lib-core`](https://github.com/zed-pkg/zed-lib-core).
- This repository may be archived after the canonical registry-data-plane port
  is merged and no unique predecessor work remains.

## License

MIT
