use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ApiResponse {
    message: String,
    id: Option<String>,
}

impl ApiResponse {
    pub fn new<S: Into<String>>(message: S, id: Option<String>) -> Self {
        ApiResponse {
            message: message.into(),
            id,
        }
    }
}
