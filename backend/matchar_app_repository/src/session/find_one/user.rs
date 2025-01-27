use database::ConnectionPool;
use matchar_app_service::session::find_one::{outbound, Error, UserPort};
use refinement::UserId;

pub struct UserRepository {
    pool: ConnectionPool,
}

impl UserRepository {
    pub const fn new(pool: ConnectionPool) -> Self {
        Self { pool }
    }
}

impl UserPort for UserRepository {
    async fn find_by_session_id(
        &self,
        session_id: refinement::SessionId,
    ) -> Result<Option<outbound::UserEntity>, Error> {
        let record = sqlx::query!(
            r#"
            SELECT
                "u"."user_id",
                "u"."deactivated_at",
                "u"."locked_at"
            FROM
                "session" "s"
            JOIN
                "user" "u"
            ON
                "s"."user_id" = "u"."user_id"
            WHERE
                "s"."session_id" = $1
            "#,
            session_id.as_uuid(),
        )
        .fetch_optional(&*self.pool)
        .await
        .map_err(|error| Error::DatabaseError(error.into()))?;

        match record {
            Some(row) => {
                let user_id = UserId::new_unchecked(row.user_id);
                let deactivated_at = row.deactivated_at.assume_utc();
                let locked_at = row.locked_at.assume_utc();

                Ok(Some(outbound::UserEntity {
                    user_id,
                    deactivated_at,
                    locked_at,
                }))
            }
            None => Ok(None),
        }
    }
}
