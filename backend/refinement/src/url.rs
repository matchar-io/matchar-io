use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[serde(transparent)]
pub struct ImageUrl(Cow<'static, str>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ImageFormat {
    Png,
    Jpeg,
    Gif,
    Webp,
}

#[derive(Debug, Error)]
pub enum ImageUrlError {
    #[error("invalid image url")]
    Invalid,
    #[error("unsupported image format")]
    Unsupported,
    #[error("invalid image url: {0}")]
    InvalidUrl(#[from] url::ParseError),
}

impl ImageUrl {
    pub const USER_DEFAULT: Self = Self(Cow::Borrowed(crate::USER_DEFAULT_IMAGE));

    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[inline]
    pub fn format(&self) -> Result<ImageFormat, ImageUrlError> {
        self.0.parse()
    }
}

impl std::fmt::Display for ImageUrl {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for ImageUrl {
    type Err = ImageUrlError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        let url = url::Url::parse(source).map_err(ImageUrlError::InvalidUrl)?;

        match ImageFormat::from_url(url) {
            Some(_) => Ok(Self(Cow::Owned(source.to_owned()))),
            None => Err(ImageUrlError::Unsupported),
        }
    }
}

impl<'de> serde::Deserialize<'de> for ImageUrl {
    fn deserialize<D>(deserializer: D) -> Result<ImageUrl, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let source = String::deserialize(deserializer)?;

        source.parse().map_err(serde::de::Error::custom)
    }
}

impl ImageFormat {
    pub fn from_url(url: url::Url) -> Option<Self> {
        url.path()
            .rsplit_once('.')
            .and_then(|(_, extension)| Self::from_extension(extension))
    }

    pub fn from_extension(source: &str) -> Option<Self> {
        match source {
            "png" => Some(Self::Png),
            "jpeg" | "jpg" => Some(Self::Jpeg),
            "gif" => Some(Self::Gif),
            "webp" => Some(Self::Webp),
            _ => None,
        }
    }
}

impl std::str::FromStr for ImageFormat {
    type Err = ImageUrlError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        match source.rsplit_once('.') {
            Some((_, extension)) => match Self::from_extension(extension) {
                Some(format) => Ok(format),
                None => Err(ImageUrlError::Unsupported),
            },
            _ => Err(ImageUrlError::Invalid),
        }
    }
}
