pub struct Data {
    pub(crate) from_url: FromUrl,
}

pub struct FromUrl(String);

#[derive(Debug)]
pub enum Error {
    FromUrl(anyhow::Error),
}

impl Data {
    pub fn new(from_url: &str) -> Result<Self, Error> {
        Ok(Self {
            from_url: from_url.parse()?,
        })
    }
}

impl FromUrl {
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::str::FromStr for FromUrl {
    type Err = Error;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        url::Url::parse(source).map_err(|error| Error::FromUrl(error.into()))?;

        Ok(Self(source.to_owned()))
    }
}
