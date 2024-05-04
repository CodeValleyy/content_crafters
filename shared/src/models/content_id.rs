use utoipa::ToSchema;

#[derive(serde::Deserialize, ToSchema)]
pub struct ContentId {
    pub id: String,
}