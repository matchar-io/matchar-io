use matchar_app_service::auth::google_callback::{Error, EventPort};
use refinement::UserId;

pub struct EventRepository;

impl EventPort for EventRepository {
    async fn login_completed(&self, _user_id: UserId) -> Result<(), Error> {
        Ok(())
    }
}
