# zed-orm-core

The requested architecture restores ORM ownership to this repository. The
previous consolidation into zed-lib-core is being reversed: orm-core may depend
on lib-core, while lib-core must remain independent of orm-core. Four servers
will import both directly and obtain organization interfaces through Zed.

This patch implements the read-context foundation only. Registry, invitation,
upload, download, licensing and search operations still reside in
[zed-lib-core/src/rust-orm](https://github.com/zed-pkg/zed-lib-core/tree/main/src/rust-orm).
Their semantic migration and consumer changes are required before publishing a
replacement package. Existing consumers must retain their reviewed pins until
that migration is complete.

Read contexts use SeaORM and a private Diesel companion. Diesel adds one
connection to the configured SeaORM pool. Acquisition is bounded, synchronous
work runs on a blocking worker, and cancellation retains its permit until that
worker finishes. Both drivers install the same schema and read-only policy;
read::connection_state and read::ping reject disagreement. Raw connections and
driver errors stay private. Database privileges remain the security boundary.

The Zed manifest declares zed-interfaces and zed-lib-core. Frozen registry
resolution has not been certified while registry.zpkg.net returns HTTP 502.
Peer TypeSpec and JSON Schema generation, business-operation migration, and
four-server middleware/rate-limit adoption remain outstanding rollout steps.

Validate with cargo fmt, cargo clippy with all targets/features and denied
warnings, default/all-feature tests, and doctests. Native builds require libpq.
Set ORM_CORE_TEST_DATABASE_URL to a disposable database containing schema
zed_pkg and run the ignored live tests to verify both drivers and DDL denial.

Historical migration evidence remains in
[zed-lib-core/PREDECESSOR_MIGRATION.md](https://github.com/zed-pkg/zed-lib-core/blob/main/PREDECESSOR_MIGRATION.md).
