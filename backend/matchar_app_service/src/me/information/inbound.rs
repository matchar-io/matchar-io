use refinement::SessionId;

pub struct Data {
    pub(crate) session_id: SessionId,
}

impl Data {
    pub const fn new(session_id: SessionId) -> Self {
        Self { session_id }
    }
}
