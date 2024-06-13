use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ApiResponse {
    message: String,
    id: Option<String>,
    code_url: Option<String>,
}

impl ApiResponse {
    pub fn new<S: Into<String>>(message: S, id: Option<String>, code_url: Option<String>) -> Self {
        ApiResponse {
            message: message.into(),
            id,
            code_url,
        }
    }
}
