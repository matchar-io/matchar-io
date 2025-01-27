#[derive(Clone)]
pub enum Message {
    Event(String),
    Close,
}

impl Message {
    pub(crate) fn event<P>(r#type: &'static str, payload: P) -> Result<Self, serde_json::Error>
    where
        P: serde::Serialize,
    {
        #[derive(Serialize)]
        pub struct Event<P> {
            pub r#type: &'static str,
            pub payload: P,
        }

        Ok(Self::Event(serde_json::to_string(&payload)?))
    }
}
