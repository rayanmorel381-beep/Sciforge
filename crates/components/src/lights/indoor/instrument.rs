#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstrumentDisplayType {
    Analog,
    Digital,
    HybridTft,
    FullDigitalCluster,
    HeadUpDisplay,
}

#[derive(Debug, Clone)]
pub struct InstrumentLight {
    pub display_type: InstrumentDisplayType,
    pub brightness_levels: u8,
    pub color_temp_k: u16,
    pub night_mode: bool,
    pub auto_brightness: bool,
    pub hud_included: bool,
}

impl InstrumentLight {
    pub fn analog() -> Self {
        Self {
            display_type: InstrumentDisplayType::Analog,
            brightness_levels: 5,
            color_temp_k: 2_800,
            night_mode: false,
            auto_brightness: false,
            hud_included: false,
        }
    }

    pub fn digital() -> Self {
        Self {
            display_type: InstrumentDisplayType::Digital,
            brightness_levels: 10,
            color_temp_k: 6_500,
            night_mode: true,
            auto_brightness: false,
            hud_included: false,
        }
    }

    pub fn hybrid_tft() -> Self {
        Self {
            display_type: InstrumentDisplayType::HybridTft,
            brightness_levels: 16,
            color_temp_k: 6_500,
            night_mode: true,
            auto_brightness: true,
            hud_included: false,
        }
    }

    pub fn full_digital_with_hud() -> Self {
        Self {
            display_type: InstrumentDisplayType::FullDigitalCluster,
            brightness_levels: 255,
            color_temp_k: 6_500,
            night_mode: true,
            auto_brightness: true,
            hud_included: true,
        }
    }
}
