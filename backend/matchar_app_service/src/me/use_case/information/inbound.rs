use refinement::SessionId;

pub struct Data {
    pub(crate) session_id: SessionId,
    pub(crate) now: time::OffsetDateTime,
}

impl Data {
    pub const fn new(session_id: SessionId, now: time::OffsetDateTime) -> Self {
        Self { session_id, now }
    }
}
