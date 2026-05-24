#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LiveAxleLocating {
    FourLink,
    WattsLinkage,
    LeafSpring,
    PanhardRod,
    TractionBar,
}

#[derive(Debug, Clone)]
pub struct DependentLiveAxle {
    pub locating: LiveAxleLocating,
    pub axle_width_mm: f64,
    pub tube_diameter_mm: f64,
    pub panhard_rod: bool,
    pub watts_linkage: bool,
    pub torque_arm: bool,
}

impl DependentLiveAxle {
    pub fn four_link(axle_width_mm: f64) -> Self {
        Self {
            locating: LiveAxleLocating::FourLink,
            axle_width_mm,
            tube_diameter_mm: 89.0,
            panhard_rod: true,
            watts_linkage: false,
            torque_arm: false,
        }
    }

    pub fn watts(axle_width_mm: f64) -> Self {
        Self {
            locating: LiveAxleLocating::WattsLinkage,
            axle_width_mm,
            tube_diameter_mm: 89.0,
            panhard_rod: false,
            watts_linkage: true,
            torque_arm: false,
        }
    }

    pub fn leaf_spring(axle_width_mm: f64) -> Self {
        Self {
            locating: LiveAxleLocating::LeafSpring,
            axle_width_mm,
            tube_diameter_mm: 76.0,
            panhard_rod: false,
            watts_linkage: false,
            torque_arm: false,
        }
    }

    pub fn traction_bar(axle_width_mm: f64) -> Self {
        Self {
            locating: LiveAxleLocating::TractionBar,
            axle_width_mm,
            tube_diameter_mm: 102.0,
            panhard_rod: true,
            watts_linkage: false,
            torque_arm: true,
        }
    }
}
