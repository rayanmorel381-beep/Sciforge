#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaillightTechnology {
    Incandescent,
    Led,
    Oled,
}

#[derive(Debug, Clone)]
pub struct Taillight {
    pub technology: TaillightTechnology,
    pub sequential: bool,
    pub dynamic: bool,
}

impl Taillight {
    pub fn standard(technology: TaillightTechnology) -> Self {
        Self { technology, sequential: false, dynamic: false }
    }

    pub fn sequential(technology: TaillightTechnology) -> Self {
        Self { technology, sequential: true, dynamic: false }
    }

    pub fn dynamic(technology: TaillightTechnology) -> Self {
        Self { technology, sequential: true, dynamic: true }
    }
}
