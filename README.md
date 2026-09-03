# zed-orm-core

Canonical private, backend-only ORM boundary for the zed-pkg organization.

This repository supersedes the historical decision to place the package in `zed-lib-core/src/rust-orm`. Executable Diesel/SeaORM entities, repositories, named database capabilities, and backend database configuration finalize here only. `zed-lib-core` remains shared code with explicit client, server, edge, and isomorphic surfaces; it may retain normalized ORM comparison evidence but never executable ORM code.

## Dual-source contract

`zed-interfaces` keeps independently authored TypeSpec and JSON Schema Draft 2020-12/OpenAPI inputs. This repository and `zed-lib-core` independently consume the same immutable source revision and produce normalized interface, persistence, SQL/catalog, and ORM evidence. Publication is blocked unless every pair agrees and the exact evidence is recorded in `artifacts/agreement.lock`.

Neither TypeSpec, JSON Schema, Diesel, nor SeaORM wins automatically. Differences stop CI, package publication, migration promotion, and consumer updates for review.

## Runtime boundary

- Default capabilities are read-only; write APIs require a separate explicit type/feature.
- Raw database connections and generic SQL do not escape the crate.
- Application startup never runs migrations or DDL.
- Named operations are bounded, tenant-aware, typed, and covered by negative-access tests.
- DPM owns reviewed migration planning/application.

## Environment and dependencies

Accept typed backend configuration inward. Use `flags-2-env` at executable boundaries and `ores-sops` plus SOPS/age for encrypted environment files. A separate `zed-env` package is justified only when multiple repositories/runtimes share one configuration schema, and it never contains values or secrets.

The root `.zpkg.toml` pins `zed-pkg/zed-interfaces` and `zed-pkg/zed-lib-core`; native metadata must resolve the same reviewed revisions. The old centralized shared-definitions package is not a product schema or ORM authority.

## Blocking visibility requirement

This repository is currently public. It must be changed to private before executable ORM consolidation or release. CI intentionally reports that live repository-metadata blocker. No credentials or sensitive schema material should be added while it remains public.
