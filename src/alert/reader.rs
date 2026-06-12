use sqlx::PgPool;

use crate::Alert;
use crate::Error;
use crate::alert::{AlertQuery, AlertRow};

impl AlertRow {
    pub(crate) async fn get_alerts(pool: &PgPool, query: AlertQuery) -> Result<Vec<Alert>, Error> {
        let rows = match (query.uid, query.limit) {
            (Some(uid), _) => {
                sqlx::query_as::<_, AlertRow>("SELECT * FROM alerts WHERE id = $1")
                    .bind(uid)
                    .fetch_all(pool)
                    .await?
            }
            (None, Some(limit)) => {
                sqlx::query_as::<_, AlertRow>(
                    "SELECT * FROM alerts ORDER BY triggered_at DESC LIMIT $1 OFFSET $2",
                )
                .bind(limit)
                .bind(query.offset.unwrap_or(0))
                .fetch_all(pool)
                .await?
            }
            (None, None) => {
                sqlx::query_as::<_, AlertRow>("SELECT * FROM alerts ORDER BY triggered_at DESC")
                    .fetch_all(pool)
                    .await?
            }
        };

        Ok(rows.into_iter().map(Alert::from).collect())
    }

    pub(crate) async fn count(pool: &PgPool) -> Result<i64, Error> {
        let (total,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM alerts")
            .fetch_one(pool)
            .await?;
        Ok(total)
    }
}
