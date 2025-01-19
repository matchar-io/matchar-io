use postbox::{Actor, Postbox};
use refinement::{ImageUrl, UserId, UserName};

pub struct UserPostbox {
    postbox: Postbox<UserActor>,
}

pub struct UserActor {
    user: User,
}

pub struct User {
    user_id: UserId,
    name: UserName,
    image_url: ImageUrl,
}

impl UserPostbox {
    #[inline]
    pub fn id(&self) -> uuid::Uuid {
        self.postbox.id()
    }
}

impl Actor for UserActor {
    //
}
