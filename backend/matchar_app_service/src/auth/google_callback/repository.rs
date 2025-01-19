use super::{inbound, outbound, Error};
use refinement::{EmailAddress, ImageUrl, PkceId, SessionId, UserId, UserName};

pub trait Repository: Sync + Send + 'static {
    type Pkce: PkceRepository;

    type User: UserRepository;

    type Session: SessionRepository;

    type Event: EventRepository;

    fn pkce(&self) -> &Self::Pkce;

    fn user(&self) -> &Self::User;

    fn session(&self) -> &Self::Session;

    fn event(&self) -> &Self::Event;
}

pub trait PkceRepository {
    type AccessToken;

    async fn find_by_csrf_token(
        &self,
        csrf_token: &inbound::CsrfToken,
    ) -> Result<Option<outbound::PkceEntity>, Error>;

    async fn verify_code(
        &self,
        pkce_id: PkceId,
        code: &inbound::Code,
        code_verifier: &outbound::CodeVerifier,
    ) -> Result<Self::AccessToken, Error>;

    async fn user_info_in_google(
        &self,
        access_token: &Self::AccessToken,
    ) -> Result<outbound::UserInGoogle, Error>;
}

pub trait UserRepository {
    async fn find_by_oauth_sub(
        &self,
        sub: &outbound::GoogleSubject,
    ) -> Result<Option<outbound::UserEntity>, Error>;

    async fn create(
        &self,
        sub: &outbound::GoogleSubject,
        email_address: &EmailAddress,
        name: &Option<UserName>,
        image_url: &Option<ImageUrl>,
    ) -> Result<outbound::UserEntity, Error>;
}

pub trait SessionRepository {
    async fn create(&self, user_id: UserId) -> Result<SessionId, Error>;
}

pub trait EventRepository {
    async fn login_completed(&self, user_id: UserId) -> Result<(), Error>;
}
