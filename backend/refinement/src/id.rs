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
                pub fn generate() -> Self {
                    Self(uuid::Uuid::now_v7())
                }

                #[inline]
                pub const fn as_u128(&self) -> u128 {
                    self.0.as_u128()
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
    Pkce -> "pkce_",
    UserId -> "user_",
    SessionId -> "session_",
}
