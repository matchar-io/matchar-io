pub struct Message {
    pub payload: String,
}

impl Message {
    pub(crate) fn new<T>(payload: T) -> Result<Self, serde_json::Error>
    where
        T: serde::Serialize,
    {
        let payload = serde_json::to_string(&payload)?;

        Ok(Self { payload })
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        &self.payload
    }
}
