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
        hood: Hood::steel(18.0),
        roof: Roof::fixed(PanelMaterial::Steel, 24.0),
        trunk: Trunk::traditional(PanelMaterial::Steel, 17.0),
        door_fl: Door::conventional(PanelMaterial::Steel, false, 22.0),
        door_fr: Door::conventional(PanelMaterial::Steel, false, 22.0),
        door_rl: Door::conventional(PanelMaterial::Steel, false, 20.0),
        door_rr: Door::conventional(PanelMaterial::Steel, false, 20.0),
        fender_front: Fender::standard(PanelMaterial::Steel, 9.0),
        fender_rear: Fender::standard(PanelMaterial::Steel, 9.0),
        spoiler: Spoiler::fixed(SpoilerMaterial::ABS, 20.0, 1145.0),
        windshield: Windshield::standard(4.6),
        side_window: SideWindow::standard(3.6),
        rear_window: RearWindow::with_wiper(4.0),
    }
}

pub fn comfort() -> BodyKit {
    BodyKit {
        hood: Hood::steel(18.0),
        roof: Roof::fixed(PanelMaterial::Steel, 24.0),
        trunk: Trunk::traditional(PanelMaterial::Steel, 17.0),
        door_fl: Door::conventional(PanelMaterial::Steel, false, 22.0),
        door_fr: Door::conventional(PanelMaterial::Steel, false, 22.0),
        door_rl: Door::conventional(PanelMaterial::Steel, false, 20.0),
        door_rr: Door::conventional(PanelMaterial::Steel, false, 20.0),
        fender_front: Fender::standard(PanelMaterial::Steel, 9.0),
        fender_rear: Fender::standard(PanelMaterial::Steel, 9.0),
        spoiler: Spoiler::fixed(SpoilerMaterial::ABS, 21.0, 1150.0),
        windshield: Windshield::acoustic(5.0),
        side_window: SideWindow::tinted(3.8),
        rear_window: RearWindow::with_wiper(4.0),
    }
}

pub fn premium() -> BodyKit {
    BodyKit {
        hood: Hood::aluminum(14.0),
        roof: Roof::panoramic(30.0),
        trunk: Trunk::power_liftgate(PanelMaterial::Steel, 18.0),
        door_fl: Door::conventional(PanelMaterial::Steel, false, 22.0),
        door_fr: Door::conventional(PanelMaterial::Steel, false, 22.0),
        door_rl: Door::conventional(PanelMaterial::Steel, false, 20.0),
        door_rr: Door::conventional(PanelMaterial::Steel, false, 20.0),
        fender_front: Fender::standard(PanelMaterial::Aluminum, 8.0),
        fender_rear: Fender::standard(PanelMaterial::Aluminum, 8.0),
        spoiler: Spoiler::fixed(SpoilerMaterial::ABS, 23.0, 1155.0),
        windshield: Windshield::acoustic(5.0),
        side_window: SideWindow::tinted(3.8),
        rear_window: RearWindow::with_wiper(4.1),
    }
}

pub fn all() -> Vec<BodyKit> {
    vec![entry(), comfort(), premium()]
}
