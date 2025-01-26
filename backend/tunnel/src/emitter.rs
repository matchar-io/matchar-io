use crate::Message;
use tokio::sync::mpsc;

#[derive(Clone)]
pub struct Emitter {
    tx: tokio::sync::mpsc::Sender<Message>,
}

#[derive(Clone)]
pub enum EmitterError {
    Serialization,
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
        let message = Message::new(r#type, payload).map_err(|_| EmitterError::Serialization)?;
        self.tx.send(message).await.map_err(EmitterError::Send)?;

        Ok(())
    }
}
