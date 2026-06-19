#![forbid(unsafe_code)]

pub(crate) mod alert;
pub(crate) mod errors;

pub use alert::Alert;
pub use alert::AlertQuery;
pub use alert::{AlertActivity, AlertEntity, AlertEvent};
pub use errors::Error;
pub use sqlx::FromRow;
pub use sqlx::PgPool;

pub async fn migrate(pool: &PgPool) -> Result<(), Error> {
    sqlx::migrate!("./migrations").run(pool).await?;
    Ok(())
}
