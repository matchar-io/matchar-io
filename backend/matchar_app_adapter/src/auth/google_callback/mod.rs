use database::ConnectionPool;
use matchar_app_service::auth::google_callback::{
    Code, CodeVerifier, CsrfToken, Error, GoogleSubject, PkceEntity, Repository, UserEntity,
    UserInfo,
};
use oauth2::{AccessToken, GoogleOauth2};
use refinement::{EmailAddress, IdentityProviderId, ImageUrl, PkceId, SessionId, UserId, UserName};
use std::str::FromStr;
use time::{Duration, OffsetDateTime, PrimitiveDateTime};

pub struct Adapter {
    pool: ConnectionPool,
    pkce: GoogleOauth2,
}

impl Adapter {
    pub fn new(pool: ConnectionPool) -> Result<Self, Error> {
        let pkce = GoogleOauth2::new(
            crate::GOOGLE_CLIENT_ID,
            crate::GOOGLE_CLIENT_SECRET,
            crate::GOOGLE_REDIRECT_URL,
        )
        .map_err(|error| Error::Oauth2(error.into()))?;

        Ok(Self { pool, pkce })
    }
}

impl Repository for Adapter {
    type AccessToken = AccessToken;

    async fn find_pkce_by_csrf_token(
        &self,
        csrf_token: &CsrfToken,
    ) -> Result<Option<PkceEntity>, Error> {
        let record = sqlx::query!(
            r#"
            SELECT
                "pkce_id",
                "code_verifier",
                "expired_at"
            FROM
                "pkce"
            WHERE
                "csrf_token" = $1
            "#,
            csrf_token.as_str()
        )
        .fetch_optional(&*self.pool)
        .await
        .map_err(|error| Error::Database(error.into()))?;

        match record {
            Some(row) => Ok(Some(PkceEntity::new(
                PkceId::new_unchecked(row.pkce_id),
                row.code_verifier,
                row.expired_at.assume_utc(),
            ))),
            None => Ok(None),
        }
    }

    async fn verify_code(
        &self,
        pkce_id: PkceId,
        code: &Code,
        code_verifier: &CodeVerifier,
    ) -> Result<Self::AccessToken, Error> {
        let access_token = self
            .pkce
            .verify(code.as_str(), code_verifier.as_str())
            .await
            .map_err(|error| Error::Verify(error.into()))?;
        let now = OffsetDateTime::now_utc();
        let expired_at = PrimitiveDateTime::new(now.date(), now.time());

        sqlx::query!(
            r#"
            UPDATE "pkce"
            SET
                "expired_at" = $1
            WHERE
                "pkce_id" = $2
            "#,
            expired_at,
            pkce_id.as_uuid(),
        )
        .execute(&*self.pool)
        .await
        .map_err(|error| Error::Database(error.into()))?;

        Ok(access_token)
    }

    async fn user_info_in_google(
        &self,
        access_token: &Self::AccessToken,
    ) -> Result<UserInfo, Error> {
        #[derive(Deserialize)]
        struct Response {
            sub: String,
            email: String,
            name: Option<String>,
            picture: Option<String>,
        }

        let response: Response = reqwest::Client::new()
            .get("https://www.googleapis.com/oauth2/v3/userinfo")
            .bearer_auth(access_token.as_str())
            .send()
            .await
            .map_err(|error| Error::Google(error.into()))?
            .json()
            .await
            .map_err(|error| Error::Google(error.into()))?;

        let sub = GoogleSubject::new(response.sub);
        let email_address = EmailAddress::new_unchecked(response.email);
        let name = match response.name.map(UserName::new) {
            Some(Ok(name)) => Some(name),
            _ => None,
        };
        let image_url = match response
            .picture
            .map(|image_url| ImageUrl::from_str(&image_url))
        {
            Some(Ok(image_url)) => Some(image_url),
            _ => None,
        };

        Ok(UserInfo {
            sub,
            email_address,
            name,
            image_url,
        })
    }

    async fn find_user_by_oauth_sub(
        &self,
        sub: &GoogleSubject,
    ) -> Result<Option<UserEntity>, Error> {
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

                Ok(Some(UserEntity {
                    user_id,
                    email_address,
                    name,
                    image_url,
                }))
            }
            None => Ok(None),
        }
    }

    async fn new_user(
        &self,
        sub: &GoogleSubject,
        email_address: &EmailAddress,
        name: &Option<UserName>,
        image_url: &Option<ImageUrl>,
    ) -> Result<UserEntity, Error> {
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
        let user = UserEntity {
            user_id,
            email_address: email_address.clone(),
            name: name.clone(),
            image_url: image_url.clone(),
        };

        sqlx::query!(
            r#"
            INSERT INTO "user" (
                "user_id"
            )
            VALUES ($1)
            "#,
            user_id.as_uuid(),
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

    async fn new_session(&self, user_id: UserId) -> Result<SessionId, Error> {
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

    async fn logged_in_event(&self, user_id: UserId) -> Result<(), Error> {
        // std::todo!();

        Ok(())
    }
}
