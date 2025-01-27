use postbox::PostOffice;

#[derive(Deserialize)]
#[serde(untagged)]
pub enum Message {
    Channel(crate::channel::command::ChannelMessage),
    Room(crate::room::command::RoomMessage),
}

impl Message {
    pub async fn tell(self, office: PostOffice) -> anyhow::Result<()> {
        match self {
            Self::Channel(message) => message.tell(office).await,
            Self::Room(message) => message.tell(office).await,
        }
    }
}

impl std::str::FromStr for Message {
    type Err = serde_json::Error;

    #[inline]
    fn from_str(source: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(source)
    }
}
