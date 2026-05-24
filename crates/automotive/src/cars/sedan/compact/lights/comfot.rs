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

pub fn comfort() -> OutdoorLightKit {
    OutdoorLightKit {
        headlight: Headlight::led(false),
        taillight: Taillight::sequential(TaillightTechnology::Led),
        drl: DaytimeRunning::integrated(),
        fog_front: FogLight::front(true),
        fog_rear: FogLight::rear(true),
        indicator_front: Indicator::led(IndicatorPosition::Front),
        indicator_rear: Indicator::led(IndicatorPosition::Rear),
        indicator_mirror: Indicator::led(IndicatorPosition::Mirror),
    }
}

pub fn premium() -> OutdoorLightKit {
    OutdoorLightKit {
        headlight: Headlight::led(true),
        taillight: Taillight::dynamic(TaillightTechnology::Led),
        drl: DaytimeRunning::adaptive(),
        fog_front: FogLight::front_cornering(true),
        fog_rear: FogLight::rear(true),
        indicator_front: Indicator::sequential(IndicatorPosition::Front),
        indicator_rear: Indicator::sequential(IndicatorPosition::Rear),
        indicator_mirror: Indicator::sequential(IndicatorPosition::Mirror),
    }
}

pub fn all() -> Vec<OutdoorLightKit> {
    vec![entry(), comfort(), premium()]
}
