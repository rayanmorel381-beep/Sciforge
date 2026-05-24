#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwingarmType {
    Single,
    Dual,
    ProLink,
    Banana,
}

#[derive(Debug, Clone)]
pub struct Swingarm {
    pub swingarm_type: SwingarmType,
    pub length_mm: f64,
    pub aluminum: bool,
    pub linkage_ratio: f64,
}

impl Swingarm {
    pub fn dual(length_mm: f64) -> Self {
        Self { swingarm_type: SwingarmType::Dual, length_mm, aluminum: false, linkage_ratio: 1.0 }
    }

    pub fn single_sided(length_mm: f64) -> Self {
        Self { swingarm_type: SwingarmType::Single, length_mm, aluminum: true, linkage_ratio: 1.0 }
    }

    pub fn pro_link(length_mm: f64, linkage_ratio: f64) -> Self {
        Self { swingarm_type: SwingarmType::ProLink, length_mm, aluminum: true, linkage_ratio }
    }
}
