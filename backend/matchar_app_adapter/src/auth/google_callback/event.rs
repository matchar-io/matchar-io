use matchar_app_service::auth::google_callback::{Error, EventRepository};
use refinement::UserId;

pub struct EventAdapter;

impl EventRepository for EventAdapter {
    async fn login_completed(&self, _user_id: UserId) -> Result<(), Error> {
        Ok(())
    }
}
