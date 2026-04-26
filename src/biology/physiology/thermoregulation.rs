use crate::constants::{
    BSA_DUBOIS_COEFF, BSA_HEIGHT_EXP, BSA_WEIGHT_EXP, HEAT_INDEX_CONSTANT,
    HEAT_INDEX_HUMIDITY_COEFF, HEAT_INDEX_HUMIDITY2, HEAT_INDEX_INTERACTION, HEAT_INDEX_T2H,
    HEAT_INDEX_T2H2, HEAT_INDEX_TEMP_COEFF, HEAT_INDEX_TEMP2, HEAT_INDEX_TH2, WINDCHILL_CONSTANT,
    WINDCHILL_INTERACTION, WINDCHILL_TEMP_COEFF, WINDCHILL_WIND_COEFF, WINDCHILL_WIND_EXP,
};

pub fn heat_balance(
    metabolic_rate: f64,
    work: f64,
    radiation: f64,
    convection: f64,
    evaporation: f64,
) -> f64 {
    metabolic_rate - work - radiation - convection - evaporation
}

pub fn newton_cooling(body_temp: f64, ambient_temp: f64, h: f64, surface_area: f64) -> f64 {
    h * surface_area * (body_temp - ambient_temp)
}

pub fn evaporative_heat_loss(sweat_rate: f64, latent_heat: f64) -> f64 {
    sweat_rate * latent_heat
}

pub fn core_temperature_regulation(
    set_point: f64,
    core_temp: f64,
    gain_shiver: f64,
    gain_sweat: f64,
) -> (f64, f64) {
    let error = core_temp - set_point;
    let shivering = if error < 0.0 {
        gain_shiver * (-error)
    } else {
        0.0
    };
    let sweating = if error > 0.0 { gain_sweat * error } else { 0.0 };
    (shivering, sweating)
}

pub fn wind_chill_index(air_temp: f64, wind_speed_kmh: f64) -> f64 {
    WINDCHILL_CONSTANT + WINDCHILL_TEMP_COEFF * air_temp
        - WINDCHILL_WIND_COEFF * wind_speed_kmh.powf(WINDCHILL_WIND_EXP)
        + WINDCHILL_INTERACTION * air_temp * wind_speed_kmh.powf(WINDCHILL_WIND_EXP)
}

pub fn heat_index(temperature_f: f64, relative_humidity: f64) -> f64 {
    let t = temperature_f;
    let r = relative_humidity;
    HEAT_INDEX_CONSTANT
        + HEAT_INDEX_TEMP_COEFF * t
        + HEAT_INDEX_HUMIDITY_COEFF * r
        + HEAT_INDEX_INTERACTION * t * r
        + HEAT_INDEX_TEMP2 * t * t
        + HEAT_INDEX_HUMIDITY2 * r * r
        + HEAT_INDEX_T2H * t * t * r
        + HEAT_INDEX_TH2 * t * r * r
        + HEAT_INDEX_T2H2 * t * t * r * r
}

pub fn body_surface_area_dubois(weight_kg: f64, height_cm: f64) -> f64 {
    BSA_DUBOIS_COEFF * weight_kg.powf(BSA_WEIGHT_EXP) * height_cm.powf(BSA_HEIGHT_EXP)
}

pub fn counter_current_heat_exchange(
    arterial_temp: f64,
    venous_temp_return: f64,
    efficiency: f64,
) -> f64 {
    arterial_temp - efficiency * (arterial_temp - venous_temp_return)
}

pub fn brown_adipose_tissue_thermogenesis(
    bat_mass: f64,
    ucp1_activity: f64,
    substrate_availability: f64,
) -> f64 {
    bat_mass * ucp1_activity * substrate_availability
}

pub fn metabolic_rate_q10(rate_ref: f64, temp: f64, temp_ref: f64, q10: f64) -> f64 {
    rate_ref * q10.powf((temp - temp_ref) / 10.0)
}
