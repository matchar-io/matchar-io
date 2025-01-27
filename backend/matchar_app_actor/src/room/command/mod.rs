pub mod enter;

use postbox::PostOffice;

#[derive(Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum RoomMessage {
    #[serde(rename = "room.enter")]
    Enter(enter::EnterMessage),
}

impl RoomMessage {
    pub async fn tell(self, office: PostOffice) -> anyhow::Result<()> {
        match self {
            Self::Enter(message) => message.tell(office).await?,
        }

        Ok(())
    }
}
