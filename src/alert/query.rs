use uuid::Uuid;

#[derive(Default)]
pub struct AlertQuery {
    pub uid: Option<Uuid>,
    pub limit: Option<i64>,
}
