use utoipa::ToSchema;

#[derive(ToSchema)]
pub struct UploadFile {
    #[schema(example = "vec![0, 1, 2, 3]")]
    pub file: Vec<u8>,
    #[schema(example = 123)]
    pub owner_id: i32,
}

#[derive(ToSchema)]
pub struct UploadGroupFile {
    #[schema(example = "vec![0, 1, 2, 3]")]
    pub file: Vec<u8>,
    #[schema(example = 123)]
    pub group_id: i32,
    #[schema(example = 123)]
    pub owner_id: i32,
}

#[derive(ToSchema)]
pub struct UploadGroup {
    #[schema(example = "vec![0, 1, 2, 3]")]
    pub file: Vec<u8>,
    #[schema(example = 123)]
    pub group_id: i32,
    #[schema(example = 123)]
    pub owner_id: i32,
    #[schema(example = 123)]
    pub message_id: Option<i32>,
}
