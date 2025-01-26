use crate::Message;
use tokio::sync::mpsc;

#[derive(Clone)]
pub struct Emitter {
    tx: tokio::sync::mpsc::Sender<Message>,
}

pub enum EmitterError {
    Serialization(serde_json::Error),
    Send(tokio::sync::mpsc::error::SendError<Message>),
}

impl Emitter {
    pub(crate) const fn new(tx: tokio::sync::mpsc::Sender<Message>) -> Self {
        Self { tx }
    }

    pub async fn emit<P>(&self, r#type: &'static str, payload: P) -> Result<(), EmitterError>
    where
        P: serde::Serialize,
    {
        let message = Message::new(r#type, payload).map_err(EmitterError::Serialization)?;
        self.tx.send(message).await.map_err(EmitterError::Send)?;

        Ok(())
    }
}
