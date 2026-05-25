use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_json::Value;
use sqlx::{FromRow, PgPool};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

use khronika::warn;

use crate::Error;

pub(crate) mod query;
pub(crate) mod reader;
pub(crate) mod writer;

pub use query::AlertQuery;

#[derive(Debug, Clone, Serialize)]
pub struct Alert {
    pub uid: Option<Uuid>,
    pub rule_id: String,
    pub title: String,
    pub level: String,
    pub event: Value,
    pub timestamp_unix: u64,
}

#[derive(Debug, Clone, FromRow)]
pub(crate) struct AlertRow {
    id: Uuid,
    rule_id: String,
    title: String,
    level: String,
    event: Value,
    triggered_at: DateTime<Utc>,
}

impl From<&Alert> for AlertRow {
    fn from(alert: &Alert) -> Self {
        Self {
            id: Uuid::new_v4(),
            rule_id: alert.rule_id.clone(),
            title: alert.title.clone(),
            level: alert.level.clone(),
            event: alert.event.clone(),
            triggered_at: DateTime::from_timestamp(alert.timestamp_unix as i64, 0)
                .unwrap_or_else(Utc::now),
        }
    }
}

impl From<AlertRow> for Alert {
    fn from(row: AlertRow) -> Self {
        Self {
            uid: Some(row.id),
            rule_id: row.rule_id,
            title: row.title,
            level: row.level,
            event: row.event,
            timestamp_unix: row.triggered_at.timestamp().max(0) as u64,
        }
    }
}

impl Alert {
    pub fn new(rule_id: String, title: String, level: String, event: Value) -> Self {
        Self::new_at(rule_id, title, level, event, SystemTime::now())
    }

    pub async fn write(&self, pool: &PgPool) -> Result<(), Error> {
        AlertRow::insert(pool, self).await
    }

    pub async fn get(pool: &PgPool, query: AlertQuery) -> Result<Vec<Self>, Error> {
        AlertRow::get_alerts(pool, query).await
    }

    pub(crate) fn new_at(
        rule_id: String,
        title: String,
        level: String,
        event: Value,
        time: SystemTime,
    ) -> Self {
        let timestamp_unix = match time.duration_since(UNIX_EPOCH) {
            Ok(duration) => duration.as_secs(),
            Err(error) => {
                warn!(
                    rule_id = rule_id,
                    "system clock is before UNIX_EPOCH ({error}), emitting alert with timestamp_unix=0"
                );
                0
            }
        };

        Self {
            uid: None,
            rule_id,
            title,
            level,
            event,
            timestamp_unix,
        }
    }
}

#[cfg(test)]
mod tests;
