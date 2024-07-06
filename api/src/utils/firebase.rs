use actix_web::Error;
use log::info;
use reqwest::Client;

pub async fn delete_file_from_firebase(
    client: &Client,
    firebase_bucket: &str,
    file_path: &str,
) -> Result<(), Error> {
    let delete_url = format!(
        "https://firebasestorage.googleapis.com/v0/b/{}/o/{}",
        firebase_bucket, file_path
    );
    let delete_response = client.delete(&delete_url).send().await;

    match delete_response {
        Ok(res) if res.status().is_success() => {
            info!("File deleted from: {:?}", file_path);
            Ok(())
        }
        Ok(res) => {
            let error_message = res
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(actix_web::error::ErrorBadRequest(format!(
                "Error deleting file: {}",
                error_message
            ))
            .into())
        }
        Err(e) => {
            Err(actix_web::error::ErrorBadRequest(format!("Error deleting file: {}", e)).into())
        }
    }
}
