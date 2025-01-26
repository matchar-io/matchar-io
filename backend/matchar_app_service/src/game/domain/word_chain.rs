use super::config::{Attribute, MaxRound, MaxTimeout};
use postbox::{Actor, Postbox};
use refinement::GameId;
use std::collections::HashSet;

pub type WordChainGamePostbox = Postbox<WordChainGame>;

pub type WordChainGameCommand = crate::common::actor::Command<WordChainGame>;

pub type WordChainGameEvent = crate::common::actor::Event<WordChainGame>;

/// 끝말잇기 게임
pub struct WordChainGame {
    /// 게임 ID
    pub(crate) game_id: GameId,
    /// 최대 라운드 수
    pub(crate) max_round: MaxRound,
    /// 최대 타임아웃 시간
    pub(crate) max_timeout: MaxTimeout,
    /// 게임 속성
    pub(crate) attributes: WordChainAttributes,
}

/// 끝말잇기 게임 속성
pub struct WordChainAttributes {
    /// 매너
    pub(crate) manner: bool,
    /// 젠틀
    pub(crate) gentle: bool,
    /// 에디켓
    pub(crate) etiquette: bool,
    /// 어인정
    pub(crate) sportsmanship: bool,
    /// 미션
    pub(crate) mission: bool,
    /// 우리말
    pub(crate) korean: bool,
    /// 깐깐
    pub(crate) picky: bool,
    /// 새내기
    pub(crate) freshman: bool,
}

impl WordChainGame {
    pub fn new(
        game_id: GameId,
        max_round: MaxRound,
        max_timeout: MaxTimeout,
        attributes: HashSet<Attribute>,
    ) -> Self {
        Self {
            game_id,
            max_round,
            max_timeout,
            attributes: WordChainAttributes::new(attributes),
        }
    }
}

impl Actor for WordChainGame {
    type Id = GameId;

    #[inline]
    fn id(&self) -> Self::Id {
        self.game_id
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
