use crate::components::powertrain::engines::thermals::assemblies::{aspiration::Aspiration, bloc::Bloc, layout::Layout};
use crate::components::powertrain::engines::thermals::parts::culasses::Culasse;
use super::cc;

const L: Layout = Layout::V;
const N: u8 = 12;

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

pub fn v12_5_0_ss_g() -> Bloc { make(cc::_5_0::CC, 83.6, 76.0, "SS_g", 10.0, 1.3) }
pub fn v12_5_0_ts_g() -> Bloc { make(cc::_5_0::CC, 83.6, 76.0, "TS_g",  9.5, 1.6) }
pub fn v12_5_0_vg_g() -> Bloc { make(cc::_5_0::CC, 83.6, 76.0, "VG_g",  8.5, 1.8) }
pub fn v12_5_5_ss_g() -> Bloc { make(cc::_5_5::CC, 86.3, 78.5, "SS_g", 10.0, 1.3) }
pub fn v12_5_5_ts_g() -> Bloc { make(cc::_5_5::CC, 86.3, 78.5, "TS_g",  9.5, 1.6) }
pub fn v12_5_5_vg_g() -> Bloc { make(cc::_5_5::CC, 86.3, 78.5, "VG_g",  8.5, 1.8) }
pub fn v12_6_0_ss_g() -> Bloc { make(cc::_6_0::CC, 88.8, 80.7, "SS_g", 10.0, 1.3) }
pub fn v12_6_0_ts_g() -> Bloc { make(cc::_6_0::CC, 88.8, 80.7, "TS_g",  9.5, 1.6) }
pub fn v12_6_0_vg_g() -> Bloc { make(cc::_6_0::CC, 88.8, 80.7, "VG_g",  8.5, 1.8) }
pub fn v12_6_2_ss_g() -> Bloc { make(cc::_6_2::CC, 89.8, 81.6, "SS_g", 10.0, 1.3) }
pub fn v12_6_2_ts_g() -> Bloc { make(cc::_6_2::CC, 89.8, 81.6, "TS_g",  9.5, 1.6) }
pub fn v12_6_2_vg_g() -> Bloc { make(cc::_6_2::CC, 89.8, 81.6, "VG_g",  8.5, 1.8) }
pub fn v12_6_3_ss_g() -> Bloc { make(cc::_6_3::CC, 90.3, 82.1, "SS_g", 10.0, 1.3) }
pub fn v12_6_3_ts_g() -> Bloc { make(cc::_6_3::CC, 90.3, 82.1, "TS_g",  9.5, 1.6) }
pub fn v12_6_3_vg_g() -> Bloc { make(cc::_6_3::CC, 90.3, 82.1, "VG_g",  8.5, 1.8) }
pub fn v12_6_5_ss_g() -> Bloc { make(cc::_6_5::CC, 91.2, 82.9, "SS_g", 10.0, 1.3) }
pub fn v12_6_5_ts_g() -> Bloc { make(cc::_6_5::CC, 91.2, 82.9, "TS_g",  9.5, 1.6) }
pub fn v12_6_5_vg_g() -> Bloc { make(cc::_6_5::CC, 91.2, 82.9, "VG_g",  8.5, 1.8) }
pub fn v12_6_75_ss_g() -> Bloc { make(cc::_6_75::CC, 92.4, 84.0, "SS_g", 10.0, 1.3) }
pub fn v12_6_75_ts_g() -> Bloc { make(cc::_6_75::CC, 92.4, 84.0, "TS_g",  9.5, 1.6) }
pub fn v12_6_75_vg_g() -> Bloc { make(cc::_6_75::CC, 92.4, 84.0, "VG_g",  8.5, 1.8) }
pub fn v12_7_0_ss_g() -> Bloc { make(cc::_7_0::CC, 93.5, 85.0, "SS_g", 10.0, 1.3) }
pub fn v12_7_0_ts_g() -> Bloc { make(cc::_7_0::CC, 93.5, 85.0, "TS_g",  9.5, 1.6) }
pub fn v12_7_0_vg_g() -> Bloc { make(cc::_7_0::CC, 93.5, 85.0, "VG_g",  8.5, 1.8) }

pub fn all() -> Vec<Bloc> {
    vec![
        v12_5_0_ss_g(), v12_5_0_ts_g(), v12_5_0_vg_g(), v12_5_5_ss_g(), v12_5_5_ts_g(), v12_5_5_vg_g(),
        v12_6_0_ss_g(), v12_6_0_ts_g(), v12_6_0_vg_g(), v12_6_2_ss_g(), v12_6_2_ts_g(), v12_6_2_vg_g(),
        v12_6_3_ss_g(), v12_6_3_ts_g(), v12_6_3_vg_g(), v12_6_5_ss_g(), v12_6_5_ts_g(), v12_6_5_vg_g(),
        v12_6_75_ss_g(), v12_6_75_ts_g(), v12_6_75_vg_g(), v12_7_0_ss_g(), v12_7_0_ts_g(), v12_7_0_vg_g(),
    ]
}
