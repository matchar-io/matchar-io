#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserName(String);

#[derive(Debug, Error)]
pub enum Error {
    #[error("Name is empty")]
    Empty,
    #[error("Name is too long")]
    TooLong,
}

impl UserName {
    pub fn new<S>(name: S) -> Result<Self, Error>
    where
        S: Into<String>,
    {
        let name = name.into();
        match name.len() {
            0 => Err(Error::Empty),
            1..=20 => Ok(Self(name)),
            _ => Err(Error::TooLong),
        }
    }

    pub fn new_unchecked<S>(name: S) -> Self
    where
        S: Into<String>,
    {
        Self(name.into())
    }

    pub fn random() -> Self {
        let now = time::OffsetDateTime::now_utc().unix_timestamp();

        Self(format!("유저{}", now))
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
