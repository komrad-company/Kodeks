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
                sqlx::query_as::<_, AlertRow>("SELECT * FROM alerts LIMIT $1")
                    .bind(limit)
                    .fetch_all(pool)
                    .await?
            }
            (None, None) => {
                sqlx::query_as::<_, AlertRow>("SELECT * FROM alerts")
                    .fetch_all(pool)
                    .await?
            }
        };

        Ok(rows.into_iter().map(Alert::from).collect())
    }
}
