/// 게임 속성
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Attribute {
    /// 매너
    Manner,
    /// 젠틀
    Gentle,
    /// 에디켓
    Etiquette,
    /// 어인정
    Sportsmanship,
    /// 미션
    Mission,
    /// 우리말
    Korean,
    /// 깐깐
    Picky,
    /// 새내기
    Freshman,
    /// 3232
    ThirtyTwo,
    /// 속담
    Proverb,
    /// 2글자 금지
    TwoCharBan,
}

impl Attribute {
    #[inline]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Attribute::Manner => "Manner",
            Attribute::Gentle => "Gentle",
            Attribute::Etiquette => "Etiquette",
            Attribute::Sportsmanship => "Sportsmanship",
            Attribute::Mission => "Mission",
            Attribute::Korean => "Korean",
            Attribute::Picky => "Picky",
            Attribute::Freshman => "Freshman",
            Attribute::ThirtyTwo => "ThirtyTwo",
            Attribute::Proverb => "Proverb",
            Attribute::TwoCharBan => "TwoCharBan",
        }
    }
}
