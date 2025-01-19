use database::ConnectionPool;
use matchar_app_service::auth::google_callback::{inbound, outbound, Error, PkceRepository};
use oauth2::{AccessToken, GoogleOauth2};
use refinement::{EmailAddress, ImageUrl, PkceId, UserName};
use std::str::FromStr;
use time::{OffsetDateTime, PrimitiveDateTime};

pub struct PkceAdapter {
    pool: ConnectionPool,
    pkce: GoogleOauth2,
}

impl PkceAdapter {
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

impl PkceRepository for PkceAdapter {
    type AccessToken = AccessToken;

    async fn find_by_csrf_token(
        &self,
        csrf_token: &inbound::CsrfToken,
    ) -> Result<Option<outbound::PkceEntity>, Error> {
        let record = sqlx::query!(
            r#"
            SELECT
                "pkce_id",
                "code_verifier",
                "from_url",
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
            Some(row) => Ok(Some(outbound::PkceEntity::new(
                PkceId::new_unchecked(row.pkce_id),
                row.code_verifier,
                row.from_url.as_str(),
                row.expired_at.assume_utc(),
            ))),
            None => Ok(None),
        }
    }

    async fn verify_code(
        &self,
        pkce_id: PkceId,
        code: &inbound::Code,
        code_verifier: &outbound::CodeVerifier,
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
    ) -> Result<outbound::UserInGoogle, Error> {
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

        let sub = outbound::GoogleSubject::new(response.sub);
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

        Ok(outbound::UserInGoogle {
            sub,
            email_address,
            name,
            image_url,
        })
    }
}
