use postbox::Actor;
use refinement::UserId;

pub type UserPostbox = crate::common::actor::Postbox<User>;

pub type UserCommand = crate::common::actor::Command<User>;

pub type UserEvent = crate::common::actor::Event<User>;

/// 유저
pub struct User {
    /// 유저 ID
    pub(crate) user_id: UserId,
    /// 이벤트 발행자
    pub(crate) emitter: tunnel::Emitter,
}

impl User {
    pub fn new(user_id: UserId, emitter: tunnel::Emitter) -> Self {
        Self { user_id, emitter }
    }
}

impl Actor for User {
    type Id = UserId;

    #[inline]
    fn id(&self) -> Self::Id {
        self.user_id
    }
}
