# Private backend-only package

This repository is the only zed-pkg repository that may finalize executable ORM code. It must be private before consolidation or release. Do not import it into browser, Flutter/mobile, desktop UI, edge, or public SDK builds.

Raw connections remain crate-private. Applications receive narrow named capabilities and never a migration or DDL runner.
