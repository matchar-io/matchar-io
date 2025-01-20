use database::ConnectionPool;
use matchar_app_service::auth::google_callback::{Error, SessionPort};
use refinement::{SessionId, UserId};
use time::{Duration, OffsetDateTime, PrimitiveDateTime};

pub struct SessionRepository {
    pool: ConnectionPool,
}

impl SessionRepository {
    pub const fn new(pool: ConnectionPool) -> Self {
        Self { pool }
    }
}

impl SessionPort for SessionRepository {
    async fn create(&self, user_id: UserId) -> Result<SessionId, Error> {
        let session_id = SessionId::random();
        let expired_at = OffsetDateTime::now_utc() + Duration::days(30);
        let expired_at = PrimitiveDateTime::new(expired_at.date(), expired_at.time());

        sqlx::query!(
            r#"
            INSERT INTO "session" (
            "session_id",
                "user_id",
                "payload",
                "expired_at"
            )
            VALUES ($1, $2, '{}', $3)
            "#,
            session_id.as_uuid(),
            user_id.as_uuid(),
            expired_at,
        )
        .execute(&*self.pool)
        .await
        .map_err(|error| Error::Database(error.into()))?;

        Ok(session_id)
    }
}
