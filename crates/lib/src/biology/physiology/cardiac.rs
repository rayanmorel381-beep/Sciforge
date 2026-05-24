use crate::constants::{MMHG_TO_PA, WINDKESSEL_SYSTOLIC_FRACTION};

pub fn frank_starling_mechanism(
    end_diastolic_volume: f64,
    contractility: f64,
    max_stroke_volume: f64,
) -> f64 {
    max_stroke_volume * (1.0 - (-contractility * end_diastolic_volume).exp())
}

pub fn stroke_volume_cardiac_output(heart_rate: f64, stroke_volume: f64) -> f64 {
    heart_rate * stroke_volume
}

pub fn ejection_fraction(stroke_volume: f64, end_diastolic_volume: f64) -> f64 {
    stroke_volume / end_diastolic_volume.max(1e-30)
}

pub fn map_calculation(systolic: f64, diastolic: f64) -> f64 {
    diastolic + (systolic - diastolic) / 3.0
}

pub fn systemic_vascular_resistance(map: f64, cvp: f64, cardiac_output: f64) -> f64 {
    (map - cvp) / cardiac_output.max(1e-30) * 80.0
}

pub fn myocardial_oxygen_consumption(heart_rate: f64, systolic_bp: f64) -> f64 {
    heart_rate * systolic_bp
}

pub fn windkessel_pressure(
    cardiac_output: f64,
    resistance: f64,
    compliance: f64,
    t: f64,
    heart_rate: f64,
) -> f64 {
    let period = 60.0 / heart_rate;
    let systolic_time = period * WINDKESSEL_SYSTOLIC_FRACTION;
    let t_mod = t % period;
    let p_mean = cardiac_output * resistance;
    if t_mod < systolic_time {
        p_mean * (1.0 + 0.5 * (std::f64::consts::PI * t_mod / systolic_time).sin())
    } else {
        let tau = resistance * compliance;
        p_mean * (-(t_mod - systolic_time) / tau).exp()
    }
}

pub fn coronary_flow_reserve(hyperemic_flow: f64, resting_flow: f64) -> f64 {
    hyperemic_flow / resting_flow.max(1e-30)
}

pub fn qt_correction_bazett(qt_ms: f64, rr_ms: f64) -> f64 {
    qt_ms / (rr_ms / 1000.0).max(1e-30).sqrt() * 1000.0_f64.sqrt()
}

pub fn cardiac_work(stroke_volume_ml: f64, mean_pressure_mmhg: f64) -> f64 {
    stroke_volume_ml * 0.001 * mean_pressure_mmhg * MMHG_TO_PA * 0.001
}

pub fn preload_recruitable_stroke_work(stroke_work: f64, edv: f64, v0: f64) -> f64 {
    stroke_work / (edv - v0).max(1e-30)
}
