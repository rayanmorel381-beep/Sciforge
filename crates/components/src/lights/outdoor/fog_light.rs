#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FogPosition {
    Front,
    Rear,
}

#[derive(Debug, Clone)]
pub struct FogLight {
    pub position: FogPosition,
    pub led: bool,
    pub cornering: bool,
}

impl FogLight {
    pub fn front(led: bool) -> Self {
        Self { position: FogPosition::Front, led, cornering: false }
    }

    pub fn front_cornering(led: bool) -> Self {
        Self { position: FogPosition::Front, led, cornering: true }
    }

    pub fn rear(led: bool) -> Self {
        Self { position: FogPosition::Rear, led, cornering: false }
    }
}
