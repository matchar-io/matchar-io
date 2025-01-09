#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct EmailAddress(email_address::EmailAddress);

impl EmailAddress {
    pub fn new_unchecked<S>(source: S) -> Self
    where
        S: Into<String>,
    {
        Self(email_address::EmailAddress::new_unchecked(source.into()))
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl std::fmt::Display for EmailAddress {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for EmailAddress {
    type Err = email_address::Error;

    #[inline]
    fn from_str(source: &str) -> Result<Self, Self::Err> {
        email_address::EmailAddress::from_str(source).map(EmailAddress)
    }
}
