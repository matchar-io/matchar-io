/// 최대 라운드 수
#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MaxRound {
    R1 = 1,
    R2 = 2,
    R3 = 3,
    R4 = 4,
    #[default]
    R5 = 5,
    R6 = 6,
    R7 = 7,
    R8 = 8,
    R9 = 9,
    R10 = 10,
}

impl MaxRound {
    pub const MIN: Self = MaxRound::R1;

    pub const MAX: Self = MaxRound::R10;

    pub const fn new(round: usize) -> Option<Self> {
        match round {
            1 => Some(Self::R1),
            2 => Some(Self::R2),
            3 => Some(Self::R3),
            4 => Some(Self::R4),
            5 => Some(Self::R5),
            6 => Some(Self::R6),
            7 => Some(Self::R7),
            8 => Some(Self::R8),
            9 => Some(Self::R9),
            10 => Some(Self::R10),
            _ => None,
        }
    }

    #[inline]
    pub const fn as_usize(self) -> usize {
        self as usize
    }
}
