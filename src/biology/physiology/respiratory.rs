use crate::constants::{DISSOLVED_O2_COEFF, HB_O2_CAPACITY, O2_DELIVERY_SCALING};

pub fn tidal_volume(respiratory_rate: f64, minute_ventilation: f64) -> f64 {
    minute_ventilation / respiratory_rate
}

pub fn alveolar_ventilation(tidal_volume: f64, dead_space: f64, rate: f64) -> f64 {
    (tidal_volume - dead_space) * rate
}

pub fn alveolar_gas_equation(fio2: f64, p_atm: f64, p_h2o: f64, paco2: f64, rq: f64) -> f64 {
    fio2 * (p_atm - p_h2o) - paco2 / rq
}

pub fn airway_resistance(pressure_drop: f64, flow: f64) -> f64 {
    pressure_drop / flow
}

pub fn lung_compliance(volume_change: f64, pressure_change: f64) -> f64 {
    volume_change / pressure_change
}

pub fn oxygen_content(hb: f64, sao2: f64, pao2: f64) -> f64 {
    HB_O2_CAPACITY * hb * sao2 + DISSOLVED_O2_COEFF * pao2
}

pub fn oxygen_delivery(cardiac_output: f64, cao2: f64) -> f64 {
    cardiac_output * cao2 * O2_DELIVERY_SCALING
}

pub fn fick_oxygen_consumption(cardiac_output: f64, cao2: f64, cvo2: f64) -> f64 {
    cardiac_output * (cao2 - cvo2) * O2_DELIVERY_SCALING
}
