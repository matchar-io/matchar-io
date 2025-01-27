use refinement::UserId;

pub struct Data {
    pub user_id: UserId,
}

pub struct UserEntity {
    pub user_id: UserId,
    pub deactivated_at: time::OffsetDateTime,
    pub locked_at: time::OffsetDateTime,
}
