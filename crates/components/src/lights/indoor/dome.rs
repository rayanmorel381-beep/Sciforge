#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DomeLightTechnology {
    Incandescent,
    Fluorescent,
    Led,
}

#[derive(Debug, Clone)]
pub struct DomeLight {
    pub technology: DomeLightTechnology,
    pub lumens: u16,
    pub color_temp_k: u16,
    pub dimming: bool,
    pub delay_off_s: f64,
    pub map_light: bool,
}

impl DomeLight {
    pub fn standard() -> Self {
        Self {
            technology: DomeLightTechnology::Incandescent,
            lumens: 60,
            color_temp_k: 2_700,
            dimming: false,
            delay_off_s: 30.0,
            map_light: false,
        }
    }

    pub fn led() -> Self {
        Self {
            technology: DomeLightTechnology::Led,
            lumens: 200,
            color_temp_k: 6_000,
            dimming: true,
            delay_off_s: 30.0,
            map_light: true,
        }
    }

    pub fn led_with_map() -> Self {
        Self {
            technology: DomeLightTechnology::Led,
            lumens: 350,
            color_temp_k: 6_000,
            dimming: true,
            delay_off_s: 60.0,
            map_light: true,
        }
    }
}
