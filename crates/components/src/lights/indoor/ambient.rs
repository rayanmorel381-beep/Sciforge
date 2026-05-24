#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AmbientZone {
    Dashboard,
    Footwell,
    DoorPanel,
    Ceiling,
    CenterConsole,
    SeatBase,
}

#[derive(Debug, Clone)]
pub struct AmbientLight {
    pub zone: AmbientZone,
    pub rgb: bool,
    pub zone_count: u8,
    pub color_temp_k: u16,
    pub max_lumens: u16,
    pub brightness_levels: u8,
    pub sync_with_music: bool,
}

impl AmbientLight {
    pub fn single(zone: AmbientZone) -> Self {
        Self {
            zone,
            rgb: false,
            zone_count: 1,
            color_temp_k: 3_000,
            max_lumens: 80,
            brightness_levels: 3,
            sync_with_music: false,
        }
    }

    pub fn rgb(zone: AmbientZone, zone_count: u8) -> Self {
        Self {
            zone,
            rgb: true,
            zone_count,
            color_temp_k: 0,
            max_lumens: 200,
            brightness_levels: 64,
            sync_with_music: false,
        }
    }

    pub fn rgb_sync(zone: AmbientZone, zone_count: u8) -> Self {
        Self {
            zone,
            rgb: true,
            zone_count,
            color_temp_k: 0,
            max_lumens: 300,
            brightness_levels: 255,
            sync_with_music: true,
        }
    }
}
