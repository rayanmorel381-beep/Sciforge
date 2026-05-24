#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DerailleurPosition {
    Front,
    Rear,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DerailleurActuation {
    Mechanical,
    Electronic,
    Wireless,
}

#[derive(Debug, Clone)]
pub struct Derailleur {
    pub position: DerailleurPosition,
    pub actuation: DerailleurActuation,
    pub max_sprocket_teeth: u8,
    pub total_capacity_teeth: u8,
}

impl Derailleur {
    pub fn mechanical_rear(max_sprocket_teeth: u8, total_capacity_teeth: u8) -> Self {
        Self { position: DerailleurPosition::Rear, actuation: DerailleurActuation::Mechanical, max_sprocket_teeth, total_capacity_teeth }
    }

    pub fn electronic_rear(max_sprocket_teeth: u8) -> Self {
        Self { position: DerailleurPosition::Rear, actuation: DerailleurActuation::Electronic, max_sprocket_teeth, total_capacity_teeth: 0 }
    }

    pub fn wireless_rear(max_sprocket_teeth: u8) -> Self {
        Self { position: DerailleurPosition::Rear, actuation: DerailleurActuation::Wireless, max_sprocket_teeth, total_capacity_teeth: 0 }
    }

    pub fn front(actuation: DerailleurActuation) -> Self {
        Self { position: DerailleurPosition::Front, actuation, max_sprocket_teeth: 0, total_capacity_teeth: 16 }
    }
}
