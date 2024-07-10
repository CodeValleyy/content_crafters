use super::error::UploadError;
use actix_web::Error;
use futures::StreamExt;

pub async fn parse_id(field_name: &str, mut field: actix_multipart::Field) -> Result<i32, Error> {
    let mut data = Vec::new();
    while let Some(chunk) = field.next().await {
        data.extend_from_slice(&chunk?);
    }
    let group_id_str = String::from_utf8(data).map_err(|_| {
        UploadError::BadRequest(format!("Invalid UTF-8 sequence in {}", field_name))
    })?;

    if field_name == "message_id" {
        if group_id_str.is_empty() {
            return Ok(0);
        }
    }

    Ok(group_id_str
        .parse::<i32>()
        .map_err(|_| UploadError::BadRequest(format!("Invalid {} format", field_name)))?)
}
