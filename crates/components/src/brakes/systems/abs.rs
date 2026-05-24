#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbsGeneration {
    FirstGen,
    ThirdGen,
    NinthGen,
    Motorcycle,
}

#[derive(Debug, Clone)]
pub struct Abs {
    pub generation: AbsGeneration,
    pub channel_count: u8,
    pub sensor_count: u8,
    pub cornering_abs: bool,
}

impl Abs {
    pub fn standard() -> Self {
        Self { generation: AbsGeneration::ThirdGen, channel_count: 4, sensor_count: 4, cornering_abs: false }
    }

    pub fn performance() -> Self {
        Self { generation: AbsGeneration::NinthGen, channel_count: 4, sensor_count: 4, cornering_abs: true }
    }

    pub fn motorcycle() -> Self {
        Self { generation: AbsGeneration::Motorcycle, channel_count: 2, sensor_count: 2, cornering_abs: false }
    }
}
