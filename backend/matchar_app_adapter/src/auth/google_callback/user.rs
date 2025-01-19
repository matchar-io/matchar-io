use database::ConnectionPool;
use matchar_app_service::auth::google_callback::{outbound, Error, UserRepository};
use refinement::{EmailAddress, IdentityProviderId, ImageUrl, UserId, UserName, ETERNITY};
use std::str::FromStr;

pub struct UserAdapter {
    pool: ConnectionPool,
}

impl UserAdapter {
    pub const fn new(pool: ConnectionPool) -> Self {
        Self { pool }
    }
}

impl UserRepository for UserAdapter {
    async fn find_by_oauth_sub(
        &self,
        sub: &outbound::GoogleSubject,
    ) -> Result<Option<outbound::UserEntity>, Error> {
        let record = sqlx::query!(
            r#"
            SELECT
                "uc"."user_id",
                "uc"."email_address",
                "up"."name",
                "up"."image_url"
            FROM
                "user_credential" "uc"
            JOIN
                "user_profile" "up"
            ON
                "uc"."user_id" = "up"."user_id"
            WHERE
                "uc"."external_id" = $1
            "#,
            sub.as_str()
        )
        .fetch_optional(&*self.pool)
        .await
        .map_err(|error| Error::Database(error.into()))?;

        match record {
            Some(row) => {
                let user_id = UserId::new_unchecked(row.user_id);
                let email_address = EmailAddress::new_unchecked(row.email_address);
                let name =
                    UserName::new(row.name).map_err(|error| Error::Database(error.into()))?;
                let image_url = ImageUrl::from_str(&row.image_url)
                    .map_err(|error| Error::Database(error.into()))?;

                Ok(Some(outbound::UserEntity {
                    user_id,
                    email_address,
                    name,
                    image_url,
                }))
            }
            None => Ok(None),
        }
    }

    async fn create(
        &self,
        sub: &outbound::GoogleSubject,
        email_address: &EmailAddress,
        name: &Option<UserName>,
        image_url: &Option<ImageUrl>,
    ) -> Result<outbound::UserEntity, Error> {
        let mut transaction = self
            .pool
            .begin()
            .await
            .map_err(|error| Error::Database(error.into()))?;

        let user_id = UserId::random();
        let email_address = email_address.clone();
        let name = match name {
            Some(name) => name.clone(),
            None => UserName::random(),
        };
        let image_url = match image_url {
            Some(image_url) => image_url.clone(),
            None => ImageUrl::USER_DEFAULT,
        };
        let user = outbound::UserEntity {
            user_id,
            email_address: email_address.clone(),
            name: name.clone(),
            image_url: image_url.clone(),
        };

        sqlx::query!(
            r#"
            INSERT INTO "user" (
                "user_id",
                "deactivated_at",
                "locked_at"
            )
            VALUES ($1, $2, $3)
            "#,
            user_id.as_uuid(),
            ETERNITY,
            ETERNITY,
        )
        .execute(&mut *transaction)
        .await
        .map_err(|error| Error::Database(error.into()))?;

        sqlx::query!(
            r#"
            INSERT INTO "user_profile" (
                "user_id",
                "name",
                "image_url"
            )
            VALUES ($1, $2, $3)
            "#,
            user_id.as_uuid(),
            user.name.as_str(),
            user.image_url.as_str(),
        )
        .execute(&mut *transaction)
        .await
        .map_err(|error| Error::Database(error.into()))?;

        sqlx::query!(
            r#"
            INSERT INTO "user_credential" (
                "user_id",
                "identity_provider_id",
                "external_id",
                "email_address"
            )
            VALUES ($1, $2, $3, $4)
            "#,
            user_id.as_uuid(),
            IdentityProviderId::GOOGLE.as_uuid(),
            sub.as_str(),
            user.email_address.as_str(),
        )
        .execute(&mut *transaction)
        .await
        .map_err(|error| Error::Database(error.into()))?;

        transaction
            .commit()
            .await
            .map_err(|error| Error::Database(error.into()))?;

        Ok(user)
    }
}
