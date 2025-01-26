macro_rules! new_id {
    (
        $(
            $(#[$meta:meta])*
            $name:ident -> $prefix:literal,
        )+
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum Id {
            $(
                $name($name),
            )+
        }

        impl Id {
            pub const fn as_uuid(&self) -> uuid::Uuid {
                match self {
                    $(
                        Id::$name(id) => id.as_uuid(),
                    )+
                }
            }

            pub const fn as_u128(&self) -> u128 {
                match self {
                    $(
                        Id::$name(id) => id.as_u128(),
                    )+
                }
            }
        }

        impl std::fmt::Display for Id {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(
                        Id::$name(id) => write!(f, "{}{}", $prefix, id),
                    )+
                }
            }
        }

        impl std::str::FromStr for Id {
            type Err = IdError;

            fn from_str(source: &str) -> Result<Self, Self::Err> {
                $(
                    if source.starts_with($prefix) {
                        let source = &source[$prefix.len()..];
                        let uuid = uuid::Uuid::parse_str(source).map_err(IdError::InvalidUuid)?;

                        return Ok(Id::$name($name(uuid)));
                    }
                )+

                Err(IdError::NoMatchPrefix)
            }
        }

        impl serde::Serialize for Id {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                self.to_string().serialize(serializer)
            }
        }

        impl<'de> serde::Deserialize<'de> for Id {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                String::deserialize(deserializer)?
                    .parse()
                    .map_err(serde::de::Error::custom)
            }
        }

        #[derive(Debug, Error)]
        pub enum IdError {
            #[error("no match prefix")]
            NoMatchPrefix,
            #[error(transparent)]
            InvalidUuid(uuid::Error),
        }

        $(
            $(#[$meta])*
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
            pub struct $name(uuid::Uuid);

            impl $name {
                pub fn random() -> Self {
                    Self(uuid::Uuid::now_v7())
                }

                pub const fn new_unchecked(uuid: uuid::Uuid) -> Self {
                    Self(uuid)
                }

                #[inline]
                pub const fn as_uuid(&self) -> uuid::Uuid {
                    self.0
                }

                #[inline]
                pub const fn as_u128(&self) -> u128 {
                    self.0.as_u128()
                }
            }

            impl From<uuid::Uuid> for $name {
                #[inline]
                fn from(uuid: uuid::Uuid) -> Self {
                    Self(uuid)
                }
            }

            impl From<$name> for uuid::Uuid {
                #[inline]
                fn from(id: $name) -> Self {
                    id.0
                }
            }

            impl std::fmt::Display for $name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}{}", $prefix, self.0)
                }
            }

            impl std::str::FromStr for $name {
                type Err = IdError;

                fn from_str(source: &str) -> Result<Self, Self::Err> {
                    match source.strip_prefix($prefix) {
                        Some(source) => uuid::Uuid::parse_str(source)
                            .map($name)
                            .map_err(IdError::InvalidUuid),
                        None => Err(IdError::NoMatchPrefix),
                    }
                }
            }

            impl serde::Serialize for $name {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    self.to_string().serialize(serializer)
                }
            }

            impl<'de> serde::Deserialize<'de> for $name {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    String::deserialize(deserializer)?
                        .parse()
                        .map_err(serde::de::Error::custom)
                }
            }
        )+
    };
}

new_id! {
    PkceId -> "pkce_",
    UserId -> "user_",
    SessionId -> "session_",
    IdentityProviderId -> "credential_",
    ChannelId -> "channel_",
    RoomId -> "room_",
    GameId -> "game_",
}

impl IdentityProviderId {
    pub const GOOGLE: Self = Self(uuid::Uuid::from_u128(
        0x01945bc2_1786_7668_8b29_20f63e8c8e0f,
    ));
}
