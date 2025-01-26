use super::config::{Attribute, MaxRound, MaxTimeout};
use postbox::Postbox;
use std::collections::HashSet;

#[derive(Clone)]
pub struct WordChainGamePostbox {
    pub(crate) postbox: Postbox<WordChainGame>,
}

pub struct WordChainGame {
    pub(crate) max_round: MaxRound,
    pub(crate) max_timeout: MaxTimeout,
    pub(crate) attributes: WordChainAttributes,
}

pub struct WordChainAttributes {
    pub(crate) manner: bool,
    pub(crate) gentle: bool,
    pub(crate) etiquette: bool,
    pub(crate) sportsmanship: bool,
    pub(crate) mission: bool,
    pub(crate) korean: bool,
    pub(crate) picky: bool,
    pub(crate) freshman: bool,
}

impl WordChainGamePostbox {
    #[inline]
    pub const fn game_id(&self) -> u64 {
        self.postbox.id()
    }
}

impl From<Postbox<WordChainGame>> for WordChainGamePostbox {
    #[inline]
    fn from(postbox: Postbox<WordChainGame>) -> Self {
        Self { postbox }
    }
}

impl WordChainGame {
    pub fn new(
        max_round: MaxRound,
        max_timeout: MaxTimeout,
        attributes: HashSet<Attribute>,
    ) -> Self {
        Self {
            max_round,
            max_timeout,
            attributes: WordChainAttributes::new(attributes),
        }
    }
}

impl WordChainAttributes {
    pub fn new(attributes: HashSet<Attribute>) -> Self {
        Self {
            manner: attributes.contains(&Attribute::Manner),
            gentle: attributes.contains(&Attribute::Gentle),
            etiquette: attributes.contains(&Attribute::Etiquette),
            sportsmanship: attributes.contains(&Attribute::Sportsmanship),
            mission: attributes.contains(&Attribute::Mission),
            korean: attributes.contains(&Attribute::Korean),
            picky: attributes.contains(&Attribute::Picky),
            freshman: attributes.contains(&Attribute::Freshman),
        }
    }
}
