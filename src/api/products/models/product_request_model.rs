use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ProductModelRequest {
    pub name: String,
    pub category_id: i32,
}
