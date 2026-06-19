use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::types::Json;
use sqlx::{FromRow, PgPool};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

use khronika::warn;

use crate::Error;

pub(crate) mod query;
pub(crate) mod reader;
pub(crate) mod writer;

pub use query::AlertQuery;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertEntity {
    pub kind: String,
    pub value: String,
    pub meta: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertActivity {
    pub at: String,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertEvent {
    pub at: String,
    pub source: String,
    pub message: String,
    pub user: Option<String>,
    pub ip: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Alert {
    pub uid: Option<Uuid>,
    pub rule_id: String,
    pub title: String,
    pub level: String,
    pub status: String,
    pub assignee: Option<String>,
    pub description: Option<String>,
    pub mitre_technique: Option<String>,
    pub event: Value,
    pub entities: Vec<AlertEntity>,
    pub activity: Vec<AlertActivity>,
    pub events: Vec<AlertEvent>,
    pub timestamp_unix: u64,
}

#[derive(Debug, Clone, FromRow)]
pub(crate) struct AlertRow {
    id: Uuid,
    rule_id: String,
    title: String,
    level: String,
    status: String,
    assignee: Option<String>,
    description: Option<String>,
    mitre_technique: Option<String>,
    event: Value,
    entities: Json<Vec<AlertEntity>>,
    activity: Json<Vec<AlertActivity>>,
    events: Json<Vec<AlertEvent>>,
    triggered_at: DateTime<Utc>,
}

impl From<&Alert> for AlertRow {
    fn from(alert: &Alert) -> Self {
        Self {
            id: Uuid::new_v4(),
            rule_id: alert.rule_id.clone(),
            title: alert.title.clone(),
            level: alert.level.clone(),
            status: alert.status.clone(),
            assignee: alert.assignee.clone(),
            description: alert.description.clone(),
            mitre_technique: alert.mitre_technique.clone(),
            event: alert.event.clone(),
            entities: Json(alert.entities.clone()),
            activity: Json(alert.activity.clone()),
            events: Json(alert.events.clone()),
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
            status: row.status,
            assignee: row.assignee,
            description: row.description,
            mitre_technique: row.mitre_technique,
            event: row.event,
            entities: row.entities.0,
            activity: row.activity.0,
            events: row.events.0,
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

    pub async fn count(pool: &PgPool) -> Result<i64, Error> {
        AlertRow::count(pool).await
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
            status: "open".to_string(),
            assignee: None,
            description: None,
            mitre_technique: None,
            event,
            entities: Vec::new(),
            activity: Vec::new(),
            events: Vec::new(),
            timestamp_unix,
        }
    }
}

#[cfg(test)]
mod tests;
