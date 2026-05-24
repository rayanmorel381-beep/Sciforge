use crate::components::frames::Unibody;
use crate::components::safety::CrumpleZone;

#[derive(Debug, Clone)]
pub struct FrameKit {
    pub unibody: Unibody,
    pub crumple_front: CrumpleZone,
    pub crumple_rear: CrumpleZone,
    pub crumple_side_left: CrumpleZone,
    pub crumple_side_right: CrumpleZone,
}

pub fn sport() -> FrameKit {
    FrameKit {
        unibody: Unibody::aluminum_intensive(27000.0, 200.0),
        crumple_front: CrumpleZone::front(135.0),
        crumple_rear: CrumpleZone::rear(85.0),
        crumple_side_left: CrumpleZone::side(55.0),
        crumple_side_right: CrumpleZone::side(55.0),
    }
}

pub fn all() -> Vec<FrameKit> {
    vec![sport()]
}
