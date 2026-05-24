use crate::components::body::{
    panels::{Door, Fender, Hood, PanelMaterial, Roof, Trunk},
    aerodynamics::{Spoiler, SpoilerMaterial},
    glazing::{RearWindow, SideWindow, Windshield},
};

#[derive(Debug, Clone)]
pub struct BodyKit {
    pub hood: Hood,
    pub roof: Roof,
    pub trunk: Trunk,
    pub door_fl: Door,
    pub door_fr: Door,
    pub door_rl: Door,
    pub door_rr: Door,
    pub fender_front: Fender,
    pub fender_rear: Fender,
    pub spoiler: Spoiler,
    pub windshield: Windshield,
    pub side_window: SideWindow,
    pub rear_window: RearWindow,
}

pub fn entry() -> BodyKit {
    BodyKit {
        hood: Hood::steel(16.0),
        roof: Roof::fixed(PanelMaterial::Steel, 22.0),
        trunk: Trunk::traditional(PanelMaterial::Steel, 15.0),
        door_fl: Door::conventional(PanelMaterial::Steel, false, 21.0),
        door_fr: Door::conventional(PanelMaterial::Steel, false, 21.0),
        door_rl: Door::conventional(PanelMaterial::Steel, false, 19.0),
        door_rr: Door::conventional(PanelMaterial::Steel, false, 19.0),
        fender_front: Fender::standard(PanelMaterial::Steel, 8.5),
        fender_rear: Fender::standard(PanelMaterial::Steel, 8.5),
        spoiler: Spoiler::fixed(SpoilerMaterial::ABS, 17.0, 1090.0),
        windshield: Windshield::standard(4.4),
        side_window: SideWindow::standard(3.5),
        rear_window: RearWindow::with_wiper(3.8),
    }
}

pub fn all() -> Vec<BodyKit> {
    vec![entry()]
}
