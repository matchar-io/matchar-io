/// 최대 타임아웃 시간
#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MaxTimeout {
    T10 = 10,
    T30 = 30,
    #[default]
    T60 = 60,
    T90 = 90,
    T120 = 120,
    T150 = 150,
}

impl MaxTimeout {
    pub const MIN: Self = MaxTimeout::T10;

    pub const MAX: Self = MaxTimeout::T150;

    pub const fn new(timeout: usize) -> Option<Self> {
        match timeout {
            10 => Some(Self::T10),
            30 => Some(Self::T30),
            60 => Some(Self::T60),
            90 => Some(Self::T90),
            120 => Some(Self::T120),
            150 => Some(Self::T150),
            _ => None,
        }
    }

    #[inline]
    pub const fn as_usize(self) -> usize {
        self as usize
    }
}
