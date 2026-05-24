use crate::components::lights::{DaytimeRunning, FogLight, Headlight, Indicator, IndicatorPosition, Taillight, TaillightTechnology};

#[derive(Debug, Clone)]
pub struct OutdoorLightKit {
    pub headlight: Headlight,
    pub taillight: Taillight,
    pub drl: DaytimeRunning,
    pub fog_front: FogLight,
    pub fog_rear: FogLight,
    pub indicator_front: Indicator,
    pub indicator_rear: Indicator,
    pub indicator_mirror: Indicator,
}

pub fn entry() -> OutdoorLightKit {
    OutdoorLightKit {
        headlight: Headlight::halogen(),
        taillight: Taillight::standard(TaillightTechnology::Led),
        drl: DaytimeRunning::standard(),
        fog_front: FogLight::front(false),
        fog_rear: FogLight::rear(false),
        indicator_front: Indicator::standard(IndicatorPosition::Front),
        indicator_rear: Indicator::standard(IndicatorPosition::Rear),
        indicator_mirror: Indicator::led(IndicatorPosition::Mirror),
    }
}

pub fn all() -> Vec<OutdoorLightKit> {
    vec![entry()]
}
