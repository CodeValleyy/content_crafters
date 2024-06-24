use utoipa::ToSchema;

#[derive(ToSchema)]
pub struct UploadFile {
    #[schema(example = "vec![0, 1, 2, 3]")]
    pub file: Vec<u8>,
    #[schema(example = 123)]
    pub owner_id: i32,
}
