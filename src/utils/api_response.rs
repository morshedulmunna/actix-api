use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub status: String,
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn new(status: &str, message: &str, data: Option<T>) -> Self {
        ApiResponse {
            status: status.to_string(),
            message: message.to_string(),
            data,
        }
    }
}
