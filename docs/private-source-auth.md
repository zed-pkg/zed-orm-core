# Private shared-defs source authentication

`zed-orm-core` intentionally depends on the private Zed package authority `oresoftware/k8s-libs-and-shared-defs`. The reviewed source repository and revision are recorded in `shared-defs.lock.json`; that provenance must not be replaced with an independently authored schema, an unreviewed vendored copy, or a credentialized repository URL.

Zed source fallback accepts private GitHub credentials through its environment-only boundary. CI must expose the approved cross-organization read credential as:

```text
ZED_PKG_GITHUB_TOKEN=<read-only credential>
```

The established fleet secret name is `FLEET_GITHUB_READ_TOKEN`; a workflow that has been provisioned with that secret should map it to `ZED_PKG_GITHUB_TOKEN` only for the Zed resolution/install step. A repository-scoped GitHub Actions `GITHUB_TOKEN` does not by itself grant read access to a private authority in another organization.

The credential must not appear in `.zpkg.toml`, `.zpkg.lock`, `shared-defs.lock.json`, repository URLs, command-line arguments, generated configuration, caches, artifacts, or logs. The package graph and exact reviewed source provenance remain identical whether authentication is required or not.

When the cross-organization secret is absent, CI must fail closed. Do not make the build green by deleting `oresoftware/k8s-libs-and-shared-defs`, substituting a public package, weakening source identity checks, or silently treating GitHub 404/403 responses as proof that the package does not exist.

Provisioning of the `zed-pkg` organization/repository secret is tracked independently in `zed-pkg/zed-infra`; once provisioned, the private-source live canary should run Zed resolution against this exact authority and revision.
