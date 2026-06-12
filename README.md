# Kodeks

> *"An alert that is not written is an attack that is not recorded."*
> — Komrad Engineering Collective, May 2026

Kodeks is the database model library of the Komrad ecosystem. It defines the canonical `Alert` type and its persistence logic against PostgreSQL. [Korelator](https://github.com/komrad-company/Korelator) writes alerts through it; [Kontrol-api](https://github.com/komrad-company/Kontrol-api) reads them back for triage. Any component that touches structured alert data goes through Kodeks.

Kodeks does not evaluate. Kodeks does not correlate. Kodeks **persists**. The detection logic belongs to the consumer.

```
Alert ──write()──► PostgreSQL alerts table
      ◄──get()──── PostgreSQL alerts table
```

---

## Usage

```rust
use kodeks::{Alert, AlertQuery};
use serde_json::json;

// Write
let alert = Alert::new(
    "rule-001".to_string(),
    "Suspicious shell spawned".to_string(),
    "high".to_string(),
    json!({"process": "bash", "pid": 1234}),
);
alert.write(&pool).await?;

// Read — second page of 20, most recent first
let alerts = Alert::get(
    &pool,
    AlertQuery { limit: Some(20), offset: Some(20), ..Default::default() },
)
.await?;
let total = Alert::count(&pool).await?;

// Read — single alert by uid
let alerts = Alert::get(&pool, AlertQuery { uid: Some(id), ..Default::default() }).await?;
```

### Public types

| Type | Role |
|---|---|
| `Alert` | Domain alert — created by the correlator, persisted to and read from PostgreSQL |
| `AlertQuery` | Read filter — `uid` for exact lookup, `limit` + `offset` for paginated scan |
| `Error` | Database errors — the caller must handle them |
| `FromRow` | Re-exported from `sqlx` — implement on custom query result structs |

### `Alert::new`

```rust
pub fn new(rule_id: String, title: String, level: String, event: Value) -> Self
```

Constructs an alert stamped at the current system time. `uid` is `None` until the alert is read back from the database.

### `Alert::write`

```rust
pub async fn write(&self, pool: &PgPool) -> Result<(), Error>
```

Persists the alert to the `alerts` table. The schema must exist — Kodeks does not run migrations.

### `Alert::get`

```rust
pub async fn get(pool: &PgPool, query: AlertQuery) -> Result<Vec<Self>, Error>
```

Reads alerts from the `alerts` table, ordered by `triggered_at` descending. `AlertQuery::uid` returns at most one result and takes precedence; otherwise `AlertQuery::limit` bounds the scan and `AlertQuery::offset` skips rows (pagination). All fields are optional — omitting them all returns every row.

### `Alert::count`

```rust
pub async fn count(pool: &PgPool) -> Result<i64, Error>
```

Returns the total number of rows in the `alerts` table. Pairs with `Alert::get` for paginated reads.

---

## Dependencies

| Crate | Purpose |
|---|---|
| `sqlx` | Async PostgreSQL driver and connection pool |
| `serde` + `serde_json` | Serialization and JSON event storage |
| `uuid` | Alert identifier generation |
| `chrono` | Timestamp handling |
| `khronika` | Structured logging — built by the collective |
| `thiserror` | Error type derivation |

---

## License

AGPL-3.0-or-later — the source remains open, as all things should be.
