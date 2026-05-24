#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComfortFeature {
    Heated,
    Ventilated,
    Massage,
    MultiContour,
}

#[derive(Debug, Clone)]
pub struct ComfortSeat {
    pub features: Vec<ComfortFeature>,
    pub memory_positions: u8,
    pub lumbar_support: bool,
    pub extendable_thigh: bool,
}

impl ComfortSeat {
    pub fn standard() -> Self {
        Self { features: vec![ComfortFeature::Heated], memory_positions: 2, lumbar_support: true, extendable_thigh: false }
    }

    pub fn executive() -> Self {
        Self { features: vec![ComfortFeature::Heated, ComfortFeature::Ventilated, ComfortFeature::Massage, ComfortFeature::MultiContour], memory_positions: 3, lumbar_support: true, extendable_thigh: true }
    }
}
