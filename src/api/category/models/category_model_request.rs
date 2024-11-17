use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CategoryModelRequest {
    pub name: String,
}
