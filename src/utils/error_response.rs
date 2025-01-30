use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}

impl ErrorResponse {
    pub fn new(status: &str, message: &str) -> Self {
        ErrorResponse {
            status: status.to_string(),
            message: message.to_string(),
        }
    }
}
