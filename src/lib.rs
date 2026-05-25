#![forbid(unsafe_code)]

pub(crate) mod alert;
pub(crate) mod errors;

pub use alert::Alert;
pub use alert::AlertQuery;
pub use errors::Error;
pub use sqlx::FromRow;
