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
        unibody: Unibody::steel(SteelGrade::Mild, 17500.0, 258.0),
        crumple_front: CrumpleZone::front(78.0),
        crumple_rear: CrumpleZone::rear(48.0),
        crumple_side_left: CrumpleZone::side(28.0),
        crumple_side_right: CrumpleZone::side(28.0),
    }
}

pub fn comfort() -> FrameKit {
    FrameKit {
        unibody: Unibody::steel(SteelGrade::HighStrength, 19500.0, 268.0),
        crumple_front: CrumpleZone::front(95.0),
        crumple_rear: CrumpleZone::rear(58.0),
        crumple_side_left: CrumpleZone::side(35.0),
        crumple_side_right: CrumpleZone::side(35.0),
    }
}

pub fn premium() -> FrameKit {
    FrameKit {
        unibody: Unibody::steel(SteelGrade::UltraHighStrength, 23000.0, 258.0),
        crumple_front: CrumpleZone::front(110.0),
        crumple_rear: CrumpleZone::rear(68.0),
        crumple_side_left: CrumpleZone::side(43.0),
        crumple_side_right: CrumpleZone::side(43.0),
    }
}

pub fn all() -> Vec<FrameKit> {
    vec![entry(), comfort(), premium()]
}
