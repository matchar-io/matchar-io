use refinement::{ImageUrl, UserId, UserName};

pub struct Data {
    pub user_id: UserId,
    pub name: UserName,
    pub image_url: ImageUrl,
}

pub struct UserEntity {
    pub user_id: UserId,
    pub name: UserName,
    pub image_url: ImageUrl,
    pub deactivated_at: time::OffsetDateTime,
    pub locked_at: time::OffsetDateTime,
}
