use crate::components::powertrain::engines::thermals::assemblies::{aspiration::Aspiration, bloc::Bloc, layout::Layout};
use crate::components::powertrain::engines::thermals::parts::culasses::Culasse;
use super::cc;

const L: Layout = Layout::V;
const N: u8 = 16;

fn make(cc_val: u32, bore: f64, stroke: f64, variant: &'static str, cr: f64, boost: f64) -> Bloc {
    Bloc {
        layout: L, cylinders: N,
        displacement_cc: cc_val, bore_mm: bore, stroke_mm: stroke,
        aspiration: Aspiration::Turbo(1), variant,
        compression_ratio: cr, max_boost_bar: boost,
        conrod_ratio_lambda: 0.30,
        design_safety_factor: 2.5,
        head_variants: vec![Culasse::ohv_2v_1p_wedge(N), Culasse::sohc_2v_1p_wedge(N), Culasse::dohc_4v_1p_pent_roof(N), Culasse::dohc_4v_2p_hemi(N), Culasse::dohc_5v_1p(N)],
    }
}

pub fn v16_6_0_ss_g() -> Bloc { make(cc::_6_0::CC, 81.2, 72.5, "SS_g", 10.0, 1.3) }
pub fn v16_6_0_ts_g() -> Bloc { make(cc::_6_0::CC, 81.2, 72.5, "TS_g",  9.5, 1.6) }
pub fn v16_6_0_vg_g() -> Bloc { make(cc::_6_0::CC, 81.2, 72.5, "VG_g",  8.5, 1.8) }
pub fn v16_6_75_ss_g() -> Bloc { make(cc::_6_75::CC, 84.4, 75.4, "SS_g", 10.0, 1.3) }
pub fn v16_6_75_ts_g() -> Bloc { make(cc::_6_75::CC, 84.4, 75.4, "TS_g",  9.5, 1.6) }
pub fn v16_6_75_vg_g() -> Bloc { make(cc::_6_75::CC, 84.4, 75.4, "VG_g",  8.5, 1.8) }
pub fn v16_7_0_ss_g() -> Bloc { make(cc::_7_0::CC, 85.4, 76.2, "SS_g", 10.0, 1.3) }
pub fn v16_7_0_ts_g() -> Bloc { make(cc::_7_0::CC, 85.4, 76.2, "TS_g",  9.5, 1.6) }
pub fn v16_7_0_vg_g() -> Bloc { make(cc::_7_0::CC, 85.4, 76.2, "VG_g",  8.5, 1.8) }
pub fn v16_7_4_ss_g() -> Bloc { make(cc::_7_4::CC, 87.0, 77.7, "SS_g", 10.0, 1.3) }
pub fn v16_7_4_ts_g() -> Bloc { make(cc::_7_4::CC, 87.0, 77.7, "TS_g",  9.5, 1.6) }
pub fn v16_7_4_vg_g() -> Bloc { make(cc::_7_4::CC, 87.0, 77.7, "VG_g",  8.5, 1.8) }
pub fn v16_8_0_ss_g() -> Bloc { make(cc::_8_0::CC, 89.3, 79.7, "SS_g", 10.0, 1.3) }
pub fn v16_8_0_ts_g() -> Bloc { make(cc::_8_0::CC, 89.3, 79.7, "TS_g",  9.5, 1.6) }
pub fn v16_8_0_vg_g() -> Bloc { make(cc::_8_0::CC, 89.3, 79.7, "VG_g",  8.5, 1.8) }
pub fn v16_8_4_ss_g() -> Bloc { make(cc::_8_4::CC, 90.8, 81.1, "SS_g", 10.0, 1.3) }
pub fn v16_8_4_ts_g() -> Bloc { make(cc::_8_4::CC, 90.8, 81.1, "TS_g",  9.5, 1.6) }
pub fn v16_8_4_vg_g() -> Bloc { make(cc::_8_4::CC, 90.8, 81.1, "VG_g",  8.5, 1.8) }
pub fn v16_9_0_ss_g() -> Bloc { make(cc::_9_0::CC, 92.9, 82.9, "SS_g", 10.0, 1.3) }
pub fn v16_9_0_ts_g() -> Bloc { make(cc::_9_0::CC, 92.9, 82.9, "TS_g",  9.5, 1.6) }
pub fn v16_9_0_vg_g() -> Bloc { make(cc::_9_0::CC, 92.9, 82.9, "VG_g",  8.5, 1.8) }
pub fn v16_13_6_ss_g() -> Bloc { make(cc::_13_6::CC, 106.6, 95.2, "SS_g", 10.0, 1.3) }
pub fn v16_13_6_ts_g() -> Bloc { make(cc::_13_6::CC, 106.6, 95.2, "TS_g",  9.5, 1.6) }
pub fn v16_13_6_vg_g() -> Bloc { make(cc::_13_6::CC, 106.6, 95.2, "VG_g",  8.5, 1.8) }

pub fn all() -> Vec<Bloc> {
    vec![
        v16_6_0_ss_g(), v16_6_0_ts_g(), v16_6_0_vg_g(), v16_6_75_ss_g(), v16_6_75_ts_g(), v16_6_75_vg_g(),
        v16_7_0_ss_g(), v16_7_0_ts_g(), v16_7_0_vg_g(), v16_7_4_ss_g(), v16_7_4_ts_g(), v16_7_4_vg_g(),
        v16_8_0_ss_g(), v16_8_0_ts_g(), v16_8_0_vg_g(), v16_8_4_ss_g(), v16_8_4_ts_g(), v16_8_4_vg_g(),
        v16_9_0_ss_g(), v16_9_0_ts_g(), v16_9_0_vg_g(), v16_13_6_ss_g(), v16_13_6_ts_g(), v16_13_6_vg_g(),
    ]
}
