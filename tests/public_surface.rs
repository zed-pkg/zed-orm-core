use std::{fs, path::PathBuf};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read(relative: &str) -> String {
    fs::read_to_string(root().join(relative))
        .unwrap_or_else(|error| panic!("failed to read {relative}: {error}"))
}

#[test]
fn raw_orm_types_are_not_reexported() {
    let library = read("src/lib.rs");
    for forbidden in [
        "pub use sea_orm",
        "pub type ReadContext = DatabaseConnection",
        "pub type WriteContext = DatabaseConnection",
    ] {
        assert!(
            !library.contains(forbidden),
            "public surface leaked {forbidden}"
        );
    }

    let connection = read("src/connection.rs");
    assert!(connection.contains("connection: DatabaseConnection"));
    assert!(connection.contains("pub(crate) fn connection"));
    assert!(!connection.contains("pub fn connection(&self)"));
}

#[test]
fn every_write_symbol_is_feature_gated() {
    let library = read("src/lib.rs");
    assert!(library.contains("#[cfg(feature = \"read-write\")]\npub mod write;"));
    assert!(library.contains("#[cfg(feature = \"read-write\")]\npub use connection"));
    assert!(library.contains("compile_fail"));

    let cargo = read("Cargo.toml");
    assert!(cargo.contains("default = [\"read-only\"]"));
    assert!(cargo.contains("read-write = [\"read-only\"]"));
}

#[test]
fn dual_source_contract_is_external_independent_and_same_org() {
    let source_lock = read("contracts/source-lock.toml");
    for contract in [
        "format = \"ores.core-source-lock/v1\"",
        "interfaces_repository = \"zed-pkg/zed-interfaces\"",
        "typespec_commit =",
        "typespec_sha256 =",
        "json_schema_commit =",
        "json_schema_sha256 =",
        "comparator_version =",
    ] {
        assert!(
            source_lock.contains(contract),
            "dual-source lock lost {contract}"
        );
    }

    let boundary = read("core-boundary.toml");
    for coordinate in [
        "repository = \"zed-pkg/zed-orm-core\"",
        "interfaces_repository = \"zed-pkg/zed-interfaces\"",
        "lib_core_repository = \"zed-pkg/zed-lib-core\"",
        "orm_core_repository = \"zed-pkg/zed-orm-core\"",
        "finalization = \"fail-closed\"",
    ] {
        assert!(boundary.contains(coordinate), "boundary lost {coordinate}");
    }

    let zpkg = read(".zpkg.toml");
    assert!(zpkg.contains("\"zed-pkg/zed-interfaces\""));
    assert!(zpkg.contains("\"zed-pkg/zed-lib-core\""));
    assert!(!zpkg.contains("k8s-libs-and-shared-defs"));
}

#[test]
fn live_denial_probe_remains_available_but_opt_in() {
    let connection = read("src/connection.rs");
    assert!(connection
        .contains("#[ignore = \"requires a dedicated ORM_CORE_TEST_DATABASE_URL database\"]"));
    assert!(connection.contains("live_read_only_context_rejects_schema_ddl"));
    assert!(connection.contains("read-only context unexpectedly executed DDL"));
}
