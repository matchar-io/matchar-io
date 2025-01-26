pub mod attribute;
pub mod max_round;
pub mod max_timeout;

pub use attribute::*;
pub use max_round::*;
pub use max_timeout::*;

use std::collections::HashSet;

/// 게임 설정
#[derive(Default)]
pub struct GameConfig {
    /// 최대 라운드 수
    pub(crate) max_round: MaxRound,
    /// 최대 타임아웃 시간
    pub(crate) max_timeout: MaxTimeout,
    /// 속성 목록
    pub(crate) attributes: HashSet<Attribute>,
}

impl GameConfig {
    pub fn new(
        max_round: usize,
        max_timeout: usize,
        attributes: HashSet<Attribute>,
    ) -> Option<Self> {
        Some(Self {
            max_round: MaxRound::new(max_round)?,
            max_timeout: MaxTimeout::new(max_timeout)?,
            attributes,
        })
    }
}
