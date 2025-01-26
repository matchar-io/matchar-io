#[derive(Clone)]
pub struct Message {
    pub r#type: &'static str,
    pub payload: String,
}

impl Message {
    pub(crate) fn new<P>(r#type: &'static str, payload: P) -> Result<Self, serde_json::Error>
    where
        P: serde::Serialize,
    {
        let payload = serde_json::to_string(&payload)?;

        Ok(Self { r#type, payload })
    }

    #[inline]
    pub fn r#type(&self) -> &'static str {
        self.r#type
    }

    #[inline]
    pub fn payload(&self) -> &str {
        &self.payload
    }
}
