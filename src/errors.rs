use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("migration error: {0}")]
    MigrationError(#[from] sqlx::migrate::MigrateError),
}
