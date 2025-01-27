pub mod enter;

use postbox::PostOffice;

#[derive(Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum ChannelMessage {
    #[serde(rename = "channel.enter")]
    Enter(enter::EnterMessage),
}

impl ChannelMessage {
    pub async fn tell(self, office: PostOffice) -> anyhow::Result<()> {
        match self {
            Self::Enter(message) => message.tell(office).await?,
        }

        Ok(())
    }
}
