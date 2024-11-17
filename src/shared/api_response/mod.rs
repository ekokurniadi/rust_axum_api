use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub status: bool,
    pub data: Option<T>,
    pub message: String,
}

impl<T> ApiResponse<T> {
    pub fn new(status: bool, data: Option<T>, message: String) -> Self {
        Self {
            status,
            data,
            message,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ApiResponseWithPagination<T> {
    pub status: bool,
    pub message: String,
    pub meta: Meta,
    pub data: Vec<T>,
}

#[derive(Serialize, Default, Debug)]
pub struct Meta {
    pub total_data: i64,
    pub per_page: i64,
    pub page: i64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RequestPaginationParam {
    pub page: i64,
    pub limit: i64,
}
