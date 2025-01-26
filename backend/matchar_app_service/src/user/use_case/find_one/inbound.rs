use refinement::UserId;

pub struct Data {
    pub(crate) user_id: UserId,
    pub(crate) now: time::OffsetDateTime,
}

impl Data {
    pub const fn new(user_id: UserId, now: time::OffsetDateTime) -> Self {
        Self { user_id, now }
    }
}
