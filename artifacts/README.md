# Dual-source parity evidence

Independent TypeSpec and JSON Schema/OpenAPI lanes write canonical JCS/NDJSON pairs under `interfaces-ir/`, `contract-ir/`, `persistence-ir/`, `sql-catalog/`, and `orm-ir/`.

Bootstrap may omit evidence but cannot release. Once any pair exists, all pairs are required. `agreement.lock` is generated only after complete equivalence.
