use crate::components::body::{
    panels::{Door, Fender, Hood, PanelMaterial, Roof, Trunk},
    aerodynamics::{Diffuser, DiffuserMaterial, Splitter, SplitterMaterial, Spoiler, SpoilerMaterial, Wing, WingMount},
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
    pub diffuser: Diffuser,
    pub splitter: Splitter,
    pub wing: Wing,
    pub windshield: Windshield,
    pub side_window: SideWindow,
    pub rear_window: RearWindow,
}

pub fn sport() -> BodyKit {
    BodyKit {
        hood: Hood::vented(PanelMaterial::CarbonFibre, 8.0),
        roof: Roof::fixed(PanelMaterial::CarbonFibre, 7.5),
        trunk: Trunk::traditional(PanelMaterial::CarbonFibre, 9.0),
        door_fl: Door::conventional(PanelMaterial::CarbonFibre, false, 14.0),
        door_fr: Door::conventional(PanelMaterial::CarbonFibre, false, 14.0),
        door_rl: Door::conventional(PanelMaterial::CarbonFibre, false, 12.0),
        door_rr: Door::conventional(PanelMaterial::CarbonFibre, false, 12.0),
        fender_front: Fender::widebody(PanelMaterial::CarbonFibre, 12.0, 9.5),
        fender_rear: Fender::widebody(PanelMaterial::CarbonFibre, 15.0, 9.5),
        spoiler: Spoiler::active(SpoilerMaterial::CarbonFibre, 85.0, 1420.0),
        diffuser: Diffuser::active(DiffuserMaterial::CarbonFibre, 380.0, 1350.0, 5),
        splitter: Splitter::with_canards(SplitterMaterial::CarbonFibre, 75.0, 1580.0, 4),
        wing: Wing::dual_element(WingMount::TrunkMounted, 1350.0, 280.0),
        windshield: Windshield::acoustic(4.6),
        side_window: SideWindow::tinted(3.5),
        rear_window: RearWindow::with_wiper(3.7),
    }
}

pub fn all() -> Vec<BodyKit> {
    vec![sport()]
}
