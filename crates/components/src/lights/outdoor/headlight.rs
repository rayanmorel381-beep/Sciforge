#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeadlightTechnology {
    Halogen,
    Hid,
    Led,
    Laser,
}

#[derive(Debug, Clone)]
pub struct Headlight {
    pub technology: HeadlightTechnology,
    pub adaptive: bool,
    pub matrix: bool,
    pub auto_leveling: bool,
    pub washers: bool,
}

impl Headlight {
    pub fn halogen() -> Self {
        Self { technology: HeadlightTechnology::Halogen, adaptive: false, matrix: false, auto_leveling: false, washers: false }
    }

    pub fn led(adaptive: bool) -> Self {
        Self { technology: HeadlightTechnology::Led, adaptive, matrix: false, auto_leveling: true, washers: false }
    }

    pub fn matrix() -> Self {
        Self { technology: HeadlightTechnology::Led, adaptive: true, matrix: true, auto_leveling: true, washers: true }
    }

    pub fn laser() -> Self {
        Self { technology: HeadlightTechnology::Laser, adaptive: true, matrix: true, auto_leveling: true, washers: true }
    }
}
