use database::ConnectionPool;
use matchar_app_service::me::information::{Error, Repository, UserEntity};
use refinement::{ImageUrl, SessionId, UserId, UserName};
use std::str::FromStr;

pub struct Adapter {
    pool: ConnectionPool,
}

impl Adapter {
    pub fn new(pool: ConnectionPool) -> Self {
        Self { pool }
    }
}

impl Repository for Adapter {
    async fn find_user_by_session_id(
        &self,
        session_id: SessionId,
    ) -> Result<Option<UserEntity>, Error> {
        let record = sqlx::query!(
            r#"
            SELECT
                "u"."user_id",
                "up"."name",
                "up"."image_url",
                "u"."deactivated_at",
                "u"."locked_at"
            FROM
                "session" "s"
            JOIN
                "user" "u"
            ON
                "s"."user_id" = "u"."user_id"
            JOIN
                "user_profile" "up"
            ON
                "u"."user_id" = "up"."user_id"
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
                let name =
                    UserName::new(row.name).map_err(|error| Error::DatabaseError(error.into()))?;
                let image_url = ImageUrl::from_str(&row.image_url)
                    .map_err(|error| Error::DatabaseError(error.into()))?;
                let deactivated_at = row.deactivated_at.assume_utc();
                let locked_at = row.locked_at.assume_utc();

                Ok(Some(UserEntity {
                    user_id,
                    name,
                    image_url,
                    deactivated_at,
                    locked_at,
                }))
            }
            None => Ok(None),
        }
    }
}
