use database::ConnectionPool;
use matchar_app_service::auth::google_authorize::{inbound, outbound, Error, SessionPort};
use refinement::PkceId;

pub struct SessionRepository {
    pool: ConnectionPool,
}

impl SessionRepository {
    pub const fn new(pool: ConnectionPool) -> Self {
        Self { pool }
    }
}

impl SessionPort for SessionRepository {
    async fn store_pkce(
        &self,
        csrf_token: &outbound::CsrfToken,
        code_verifier: &outbound::CodeVerifier,
        from_url: inbound::FromUrl,
    ) -> Result<(), Error> {
        let pkce_id = PkceId::random();
        let expired_at = time::OffsetDateTime::now_utc() + time::Duration::minutes(10);
        let expired_at = time::PrimitiveDateTime::new(expired_at.date(), expired_at.time());

        sqlx::query!(
            r#"
            INSERT INTO "pkce" ("pkce_id", "csrf_token", "code_verifier", "from_url", "expired_at")
            VALUES ($1, $2, $3, $4, $5)
            "#,
            pkce_id.as_uuid(),
            csrf_token.as_str(),
            code_verifier.as_str(),
            from_url.as_str(),
            expired_at,
        )
        .execute(&*self.pool)
        .await
        .map_err(|error| Error::StorePkce(error.into()))?;

        Ok(())
    }
}
