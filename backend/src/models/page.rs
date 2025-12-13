use serde::Serialize;

#[derive(Serialize)]
pub struct Page<T> {
    pub limit: u64,
    pub offset: u64,
    pub total: u64,
    pub data: Vec<T>,
}
