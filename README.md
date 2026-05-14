# Kodeks

> *"An alert that is not written is an attack that is not recorded."*
> — Komrad Engineering Collective, May 2026

Kodeks is the database model library of the Komrad ecosystem. It defines the canonical `Alert` type and its persistence logic against PostgreSQL. Consumed by [Korelator](https://github.com/komrad-company/Korelator) — any component that must persist structured alert data goes through Kodeks.

Kodeks does not evaluate. Kodeks does not correlate. Kodeks **persists**. The detection logic belongs to the consumer.

```
Alert ──write()──► PostgreSQL alerts table
```

---

## Usage

```rust
use kodeks::{Alert, RuleLevel};
use serde_json::json;

let alert = Alert::new(
    "rule-001".to_string(),
    "Suspicious shell spawned".to_string(),
    &RuleLevel::High,
    json!({"process": "bash", "pid": 1234}),
);

alert.write(&pool).await?;
```

### Public types

| Type | Role |
|---|---|
| `Alert` | Domain alert — created by the correlator, persisted to PostgreSQL |
| `RuleLevel` | Re-exported from Kompiler — severity level of the triggering rule |
| `Error` | Database errors — the caller must handle them |
| `FromRow` | Re-exported from `sqlx` — implement on custom query result structs |

### `Alert::new`

```rust
pub fn new(rule_id: String, title: String, level: &RuleLevel, event: Value) -> Self
```

Constructs an alert stamped at the current system time.

### `Alert::write`

```rust
pub async fn write(&self, pool: &PgPool) -> Result<(), Error>
```

Persists the alert to the `alerts` table. The schema must exist — Kodeks does not run migrations.

---

## Dependencies

| Crate | Purpose |
|---|---|
| `sqlx` | Async PostgreSQL driver and connection pool |
| `serde` + `serde_json` | Serialization and JSON event storage |
| `uuid` | Alert identifier generation |
| `chrono` | Timestamp handling |
| `kompiler` | `RuleLevel` type for alert severity |
| `khronika` | Structured logging |
| `thiserror` | Error type derivation |

---

## License

AGPL-3.0-or-later — the source remains open, as all things should be.
