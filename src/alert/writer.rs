use sqlx::PgPool;

use crate::Alert;
use crate::Error;
use crate::alert::AlertRow;

impl AlertRow {
    pub(crate) async fn insert(pool: &PgPool, alert: &Alert) -> Result<(), Error> {
        let row = AlertRow::from(alert);
        sqlx::query(
            "INSERT INTO alerts
                (id, rule_id, title, level, status, assignee, description, mitre_technique,
                 event, entities, activity, events, triggered_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)",
        )
        .bind(row.id)
        .bind(&row.rule_id)
        .bind(&row.title)
        .bind(&row.level)
        .bind(&row.status)
        .bind(&row.assignee)
        .bind(&row.description)
        .bind(&row.mitre_technique)
        .bind(&row.event)
        .bind(&row.entities)
        .bind(&row.activity)
        .bind(&row.events)
        .bind(row.triggered_at)
        .execute(pool)
        .await?;
        Ok(())
    }
}
