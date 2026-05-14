#![forbid(unsafe_code)]

pub(crate) mod alert;
pub(crate) mod errors;

pub use alert::Alert;
pub use errors::Error;
pub use kompiler::RuleLevel;
pub use sqlx::FromRow;
