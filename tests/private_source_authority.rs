use std::{fs, path::PathBuf};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read(relative: &str) -> String {
    fs::read_to_string(root().join(relative))
        .unwrap_or_else(|error| panic!("failed to read {relative}: {error}"))
}

#[test]
fn private_shared_defs_authority_is_exact_and_not_credentialized() {
    let lock = read("shared-defs.lock.json");
    assert!(lock.contains(
        "\"repository\": \"https://github.com/ORESoftware/k8s-libs-and-shared-defs\""
    ));
    assert!(lock.contains(
        "\"revision\": \"c8bdc06d74746acc6439f9527ebd02697fdf028b\""
    ));
    assert!(lock.contains("\"org_slice\": \"zed-pkg\""));
    assert!(lock.contains("\"schema\": \"zed_pkg\""));
    assert!(lock.contains("pg-defs/generated/rust/sea-orm"));

    for forbidden in [
        "github.com@",
        "https://token@",
        "access_token=",
        "Authorization:",
        "?token=",
    ] {
        assert!(
            !lock.contains(forbidden),
            "private source authority contains credential material/pattern: {forbidden}"
        );
    }
}

#[test]
fn zed_manifest_keeps_the_private_dependency_instead_of_vendoring_around_it() {
    let zpkg = read(".zpkg.toml");
    assert!(zpkg.contains(
        "\"oresoftware/k8s-libs-and-shared-defs\" = \"^0.2.0\""
    ));
    assert!(!zpkg.contains("k8s-libs-and-shared-defs = { path"));
    assert!(!zpkg.contains("token"));
}

#[test]
fn credential_delivery_is_documented_as_environment_only() {
    let docs = read("docs/private-source-auth.md");
    for required in [
        "ZED_PKG_GITHUB_TOKEN",
        "FLEET_GITHUB_READ_TOKEN",
        "environment",
        "cross-organization",
        "fail closed",
    ] {
        assert!(docs.contains(required), "private source docs lost {required}");
    }
    for forbidden in ["https://TOKEN@github.com", "?token=", "access_token="] {
        assert!(!docs.contains(forbidden), "docs recommend unsafe token transport: {forbidden}");
    }
}
