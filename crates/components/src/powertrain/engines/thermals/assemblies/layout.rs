#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layout {
    Inline,
    Flat,
    V,
    W,
    Vr,
    Radial,
}

impl Layout {
    pub fn code(self) -> char {
        match self {
            Layout::Inline => 'I',
            Layout::Flat => 'F',
            Layout::V => 'V',
            Layout::W => 'W',
            Layout::Vr => 'X',
            Layout::Radial => 'R',
        }
    }

    pub fn from_u8(v: u8) -> Self {
        match v {
            1 => Layout::Flat,
            2 => Layout::V,
            3 => Layout::W,
            4 => Layout::Vr,
            _ => Layout::Inline,
        }
    }
}
