use database::ConnectionPool;
use matchar_app_service::auth::google_authorize::{
    CodeVerifier, CsrfToken, Error, Pkce, Repository,
};
use oauth2::GoogleOauth2;
use refinement::PkceId;

pub struct Adapter {
    pool: ConnectionPool,
}

impl Adapter {
    pub fn new(pool: ConnectionPool) -> Self {
        Self { pool }
    }
}

impl Repository for Adapter {
    fn new_pkce(&self) -> Result<Pkce, Error> {
        let pkce = GoogleOauth2::new(
            crate::GOOGLE_CLIENT_ID,
            crate::GOOGLE_CLIENT_SECRET,
            crate::GOOGLE_REDIRECT_URL,
        )
        .map_err(|error| Error::Pkce(error.into()))?
        .start();

        Ok(Pkce::new(
            pkce.authorize_url,
            pkce.csrf_token,
            pkce.code_verifier,
        ))
    }

    async fn new_pkce_session(
        &self,
        csrf_token: &CsrfToken,
        code_verifier: &CodeVerifier,
    ) -> Result<(), Error> {
        let pkce_id = PkceId::random();
        let expired_at = time::OffsetDateTime::now_utc() + time::Duration::minutes(10);
        let expired_at = time::PrimitiveDateTime::new(expired_at.date(), expired_at.time());

        sqlx::query!(
            r#"
            INSERT INTO "pkce" ("pkce_id", "csrf_token", "code_verifier", "expired_at")
            VALUES ($1, $2, $3, $4)
            "#,
            pkce_id.as_uuid(),
            csrf_token.as_str(),
            code_verifier.as_str(),
            expired_at,
        )
        .execute(&*self.pool)
        .await
        .map_err(|error| Error::Pkce(error.into()))?;

        Ok(())
    }
}
