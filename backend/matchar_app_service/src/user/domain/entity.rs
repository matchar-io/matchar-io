use postbox::Actor;
use refinement::UserId;

pub type UserPostbox = crate::common::postbox::Postbox<UserActor>;

pub type UserCommand = crate::common::postbox::Command<UserActor>;

pub type UserEvent = crate::common::postbox::Event<UserActor>;

/// 유저
pub struct UserActor {
    /// 유저 ID
    pub(crate) user_id: UserId,
    /// 이벤트 발행자
    pub(crate) emitter: tunnel::Emitter,
}

impl UserActor {
    pub fn new(user_id: UserId, emitter: tunnel::Emitter) -> Self {
        Self { user_id, emitter }
    }
}

impl Actor for UserActor {
    type Id = UserId;

    #[inline]
    fn id(&self) -> Self::Id {
        self.user_id
    }
}
