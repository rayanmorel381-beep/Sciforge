#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Aspiration {
    Na,
    Turbo(u8),
    TwinCharged(u8),
    Supercharged,
}

impl Aspiration {
    pub fn code(self) -> char {
        match self {
            Aspiration::Na => 'N',
            Aspiration::Turbo(_) => 'T',
            Aspiration::TwinCharged(_) => 'C',
            Aspiration::Supercharged => 'S',
        }
    }

    pub fn turbo_count(self) -> u8 {
        match self {
            Aspiration::Turbo(n) | Aspiration::TwinCharged(n) => n,
            _ => 0,
        }
    }

    pub fn has_supercharger(self) -> bool {
        matches!(self, Aspiration::Supercharged | Aspiration::TwinCharged(_))
    }

    pub fn is_forced(self) -> bool {
        !matches!(self, Aspiration::Na)
    }
}
