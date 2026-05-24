use crate::components::frames::{SteelGrade, Unibody};
use crate::components::safety::CrumpleZone;

#[derive(Debug, Clone)]
pub struct FrameKit {
    pub unibody: Unibody,
    pub crumple_front: CrumpleZone,
    pub crumple_rear: CrumpleZone,
    pub crumple_side_left: CrumpleZone,
    pub crumple_side_right: CrumpleZone,
}

pub fn entry() -> FrameKit {
    FrameKit {
        unibody: Unibody::steel(SteelGrade::Mild, 16500.0, 255.0),
        crumple_front: CrumpleZone::front(75.0),
        crumple_rear: CrumpleZone::rear(45.0),
        crumple_side_left: CrumpleZone::side(25.0),
        crumple_side_right: CrumpleZone::side(25.0),
    }
}

pub fn comfort() -> FrameKit {
    FrameKit {
        unibody: Unibody::steel(SteelGrade::HighStrength, 18500.0, 265.0),
        crumple_front: CrumpleZone::front(90.0),
        crumple_rear: CrumpleZone::rear(55.0),
        crumple_side_left: CrumpleZone::side(32.0),
        crumple_side_right: CrumpleZone::side(32.0),
    }
}

pub fn premium() -> FrameKit {
    FrameKit {
        unibody: Unibody::steel(SteelGrade::UltraHighStrength, 22000.0, 255.0),
        crumple_front: CrumpleZone::front(105.0),
        crumple_rear: CrumpleZone::rear(65.0),
        crumple_side_left: CrumpleZone::side(40.0),
        crumple_side_right: CrumpleZone::side(40.0),
    }
}

pub fn all() -> Vec<FrameKit> {
    vec![entry(), comfort(), premium()]
}
