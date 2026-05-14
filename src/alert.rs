use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_json::Value;
use sqlx::{FromRow, PgPool};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

use khronika::warn;
use kompiler::RuleLevel;

pub(crate) mod writer;

#[derive(Debug, Clone, Serialize)]
pub struct Alert {
    pub rule_id: String,
    pub title: String,
    pub level: RuleLevel,
    pub event: Value,
    pub timestamp_unix: u64,
}

#[derive(Debug, Clone, FromRow)]
pub(crate) struct AlertRow {
    pub(super) id: Uuid,
    pub(super) rule_id: String,
    pub(super) title: String,
    pub(super) level: String,
    pub(super) event: Value,
    pub(super) triggered_at: DateTime<Utc>,
}

impl From<&Alert> for AlertRow {
    fn from(alert: &Alert) -> Self {
        Self {
            id: Uuid::new_v4(),
            rule_id: alert.rule_id.clone(),
            title: alert.title.clone(),
            level: alert.level.to_string(),
            event: alert.event.clone(),
            triggered_at: DateTime::from_timestamp(alert.timestamp_unix as i64, 0)
                .unwrap_or_else(Utc::now),
        }
    }
}

impl Alert {
    pub fn new(rule_id: String, title: String, level: &RuleLevel, event: Value) -> Self {
        Self::new_at(rule_id, title, level, event, SystemTime::now())
    }

    pub async fn write(&self, pool: &PgPool) -> Result<(), crate::Error> {
        AlertRow::insert(pool, self).await
    }

    pub(crate) fn new_at(
        rule_id: String,
        title: String,
        level: &RuleLevel,
        event: Value,
        time: SystemTime,
    ) -> Self {
        let timestamp_unix = match time.duration_since(UNIX_EPOCH) {
            Ok(d) => d.as_secs(),
            Err(e) => {
                warn!(
                    rule_id = rule_id,
                    "system clock is before UNIX_EPOCH ({e}), emitting alert with timestamp_unix=0"
                );
                0
            }
        };

        Self {
            rule_id,
            title,
            level: level.clone(),
            event,
            timestamp_unix,
        }
    }
}

#[cfg(test)]
mod tests;
