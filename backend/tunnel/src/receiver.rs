use crate::Message;

pub struct Receiver {
    rx: tokio::sync::mpsc::Receiver<Message>,
    capacity: usize,
}

impl Receiver {
    pub(crate) const fn new(rx: tokio::sync::mpsc::Receiver<Message>, capacity: usize) -> Self {
        Self { rx, capacity }
    }

    #[inline]
    pub async fn message(&mut self) -> Option<Message> {
        self.rx.recv().await
    }

    #[inline]
    pub async fn messages(&mut self, buffer: &mut Vec<Message>) -> usize {
        self.rx.recv_many(buffer, self.capacity).await
    }
}
