use refinement::{EmailAddress, ImageUrl, PkceId, SessionId, UserId, UserName};

pub struct Data {
    pub session_id: SessionId,
    pub name: UserName,
    pub image_url: ImageUrl,
    pub from_url: FromUrl,
}

pub struct FromUrl(String);

pub struct PkceEntity {
    pub pkce_id: PkceId,
    pub code_verifier: CodeVerifier,
    pub from_url: FromUrl,
    pub expired_at: time::OffsetDateTime,
}

pub struct CodeVerifier(String);

pub struct UserInGoogle {
    pub sub: GoogleSubject,
    pub email_address: EmailAddress,
    pub name: Option<UserName>,
    pub image_url: Option<ImageUrl>,
}

pub struct GoogleSubject(String);

pub struct UserEntity {
    pub user_id: UserId,
    pub email_address: EmailAddress,
    pub name: UserName,
    pub image_url: ImageUrl,
}

pub struct UserToken(pub String);

impl FromUrl {
    pub const fn new(from_url: String) -> Self {
        Self(from_url)
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl CodeVerifier {
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl GoogleSubject {
    pub const fn new(sub: String) -> Self {
        Self(sub)
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl PkceEntity {
    pub fn new(
        pkce_id: PkceId,
        code_verifier: String,
        from_url: &str,
        expired_at: time::OffsetDateTime,
    ) -> Self {
        Self {
            pkce_id,
            code_verifier: CodeVerifier(code_verifier.to_owned()),
            from_url: FromUrl(from_url.to_owned()),
            expired_at,
        }
    }
}
