use database::ConnectionPool;
use matchar_app_service::user::information::{outbound, Error, UserPort};
use refinement::{ImageUrl, UserId, UserName};
use std::str::FromStr;

pub struct UserRepository {
    pool: ConnectionPool,
}

impl UserRepository {
    pub const fn new(pool: ConnectionPool) -> Self {
        Self { pool }
    }
}

impl UserPort for UserRepository {
    async fn find_by_user_id(
        &self,
        user_id: refinement::UserId,
    ) -> Result<Option<outbound::UserEntity>, Error> {
        let record = sqlx::query!(
            r#"
            SELECT
                "u"."user_id",
                "up"."name",
                "up"."image_url",
                "u"."deactivated_at",
                "u"."locked_at"
            FROM
                "user" "u"
            JOIN
                "user_profile" "up"
            ON
                "u"."user_id" = "up"."user_id"
            WHERE
                "u"."user_id" = $1
            "#,
            user_id.as_uuid(),
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

                Ok(Some(outbound::UserEntity {
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
