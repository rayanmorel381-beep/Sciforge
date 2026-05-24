use crate::components::powertrain::engines::thermals::assemblies::{aspiration::Aspiration, bloc::Bloc, layout::Layout};
use crate::components::powertrain::engines::thermals::parts::culasses::Culasse;
use super::cc;

const L: Layout = Layout::V;
const N: u8 = 8;

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

pub fn v8_3_5_ss_g() -> Bloc { make(cc::_3_5::CC, 84.4, 78.1, "SS_g", 10.0, 1.3) }
pub fn v8_3_5_ts_g() -> Bloc { make(cc::_3_5::CC, 84.4, 78.1, "TS_g",  9.5, 1.6) }
pub fn v8_3_5_vg_g() -> Bloc { make(cc::_3_5::CC, 84.4, 78.1, "VG_g",  8.5, 1.8) }
pub fn v8_3_9_ss_g() -> Bloc { make(cc::_3_9::CC, 87.5, 81.0, "SS_g", 10.0, 1.3) }
pub fn v8_3_9_ts_g() -> Bloc { make(cc::_3_9::CC, 87.5, 81.0, "TS_g",  9.5, 1.6) }
pub fn v8_3_9_vg_g() -> Bloc { make(cc::_3_9::CC, 87.5, 81.0, "VG_g",  8.5, 1.8) }
pub fn v8_4_0_ss_g() -> Bloc { make(cc::_4_0::CC, 88.3, 81.8, "SS_g", 10.0, 1.3) }
pub fn v8_4_0_ts_g() -> Bloc { make(cc::_4_0::CC, 88.3, 81.8, "TS_g",  9.5, 1.6) }
pub fn v8_4_0_vg_g() -> Bloc { make(cc::_4_0::CC, 88.3, 81.8, "VG_g",  8.5, 1.8) }
pub fn v8_4_4_ss_g() -> Bloc { make(cc::_4_4::CC, 91.1, 84.4, "SS_g", 10.0, 1.3) }
pub fn v8_4_4_ts_g() -> Bloc { make(cc::_4_4::CC, 91.1, 84.4, "TS_g",  9.5, 1.6) }
pub fn v8_4_4_vg_g() -> Bloc { make(cc::_4_4::CC, 91.1, 84.4, "VG_g",  8.5, 1.8) }
pub fn v8_4_5_ss_g() -> Bloc { make(cc::_4_5::CC, 91.8, 85.0, "SS_g", 10.0, 1.3) }
pub fn v8_4_5_ts_g() -> Bloc { make(cc::_4_5::CC, 91.8, 85.0, "TS_g",  9.5, 1.6) }
pub fn v8_4_5_vg_g() -> Bloc { make(cc::_4_5::CC, 91.8, 85.0, "VG_g",  8.5, 1.8) }
pub fn v8_4_6_ss_g() -> Bloc { make(cc::_4_6::CC, 92.5, 85.6, "SS_g", 10.0, 1.3) }
pub fn v8_4_6_ts_g() -> Bloc { make(cc::_4_6::CC, 92.5, 85.6, "TS_g",  9.5, 1.6) }
pub fn v8_4_6_vg_g() -> Bloc { make(cc::_4_6::CC, 92.5, 85.6, "VG_g",  8.5, 1.8) }
pub fn v8_4_8_ss_g() -> Bloc { make(cc::_4_8::CC, 93.8, 86.9, "SS_g", 10.0, 1.3) }
pub fn v8_4_8_ts_g() -> Bloc { make(cc::_4_8::CC, 93.8, 86.9, "TS_g",  9.5, 1.6) }
pub fn v8_4_8_vg_g() -> Bloc { make(cc::_4_8::CC, 93.8, 86.9, "VG_g",  8.5, 1.8) }
pub fn v8_5_0_ss_g() -> Bloc { make(cc::_5_0::CC, 95.1, 88.1, "SS_g", 10.0, 1.3) }
pub fn v8_5_0_ts_g() -> Bloc { make(cc::_5_0::CC, 95.1, 88.1, "TS_g",  9.5, 1.6) }
pub fn v8_5_0_vg_g() -> Bloc { make(cc::_5_0::CC, 95.1, 88.1, "VG_g",  8.5, 1.8) }
pub fn v8_5_3_ss_g() -> Bloc { make(cc::_5_3::CC, 96.9, 89.7, "SS_g", 10.0, 1.3) }
pub fn v8_5_3_ts_g() -> Bloc { make(cc::_5_3::CC, 96.9, 89.7, "TS_g",  9.5, 1.6) }
pub fn v8_5_3_vg_g() -> Bloc { make(cc::_5_3::CC, 96.9, 89.7, "VG_g",  8.5, 1.8) }
pub fn v8_5_5_ss_g() -> Bloc { make(cc::_5_5::CC, 98.1, 90.8, "SS_g", 10.0, 1.3) }
pub fn v8_5_5_ts_g() -> Bloc { make(cc::_5_5::CC, 98.1, 90.8, "TS_g",  9.5, 1.6) }
pub fn v8_5_5_vg_g() -> Bloc { make(cc::_5_5::CC, 98.1, 90.8, "VG_g",  8.5, 1.8) }
pub fn v8_5_7_ss_g() -> Bloc { make(cc::_5_7::CC, 99.3, 91.9, "SS_g", 10.0, 1.3) }
pub fn v8_5_7_ts_g() -> Bloc { make(cc::_5_7::CC, 99.3, 91.9, "TS_g",  9.5, 1.6) }
pub fn v8_5_7_vg_g() -> Bloc { make(cc::_5_7::CC, 99.3, 91.9, "VG_g",  8.5, 1.8) }
pub fn v8_6_0_ss_g() -> Bloc { make(cc::_6_0::CC, 101.0, 93.5, "SS_g", 10.0, 1.3) }
pub fn v8_6_0_ts_g() -> Bloc { make(cc::_6_0::CC, 101.0, 93.5, "TS_g",  9.5, 1.6) }
pub fn v8_6_0_vg_g() -> Bloc { make(cc::_6_0::CC, 101.0, 93.5, "VG_g",  8.5, 1.8) }
pub fn v8_6_2_ss_g() -> Bloc { make(cc::_6_2::CC, 102.1, 94.5, "SS_g", 10.0, 1.3) }
pub fn v8_6_2_ts_g() -> Bloc { make(cc::_6_2::CC, 102.1, 94.5, "TS_g",  9.5, 1.6) }
pub fn v8_6_2_vg_g() -> Bloc { make(cc::_6_2::CC, 102.1, 94.5, "VG_g",  8.5, 1.8) }
pub fn v8_6_4_ss_g() -> Bloc { make(cc::_6_4::CC, 103.2, 95.6, "SS_g", 10.0, 1.3) }
pub fn v8_6_4_ts_g() -> Bloc { make(cc::_6_4::CC, 103.2, 95.6, "TS_g",  9.5, 1.6) }
pub fn v8_6_4_vg_g() -> Bloc { make(cc::_6_4::CC, 103.2, 95.6, "VG_g",  8.5, 1.8) }
pub fn v8_7_0_ss_g() -> Bloc { make(cc::_7_0::CC, 106.4, 98.5, "SS_g", 10.0, 1.3) }
pub fn v8_7_0_ts_g() -> Bloc { make(cc::_7_0::CC, 106.4, 98.5, "TS_g",  9.5, 1.6) }
pub fn v8_7_0_vg_g() -> Bloc { make(cc::_7_0::CC, 106.4, 98.5, "VG_g",  8.5, 1.8) }

pub fn all() -> Vec<Bloc> {
    vec![
        v8_3_5_ss_g(), v8_3_5_ts_g(), v8_3_5_vg_g(), v8_3_9_ss_g(), v8_3_9_ts_g(), v8_3_9_vg_g(),
        v8_4_0_ss_g(), v8_4_0_ts_g(), v8_4_0_vg_g(), v8_4_4_ss_g(), v8_4_4_ts_g(), v8_4_4_vg_g(),
        v8_4_5_ss_g(), v8_4_5_ts_g(), v8_4_5_vg_g(), v8_4_6_ss_g(), v8_4_6_ts_g(), v8_4_6_vg_g(),
        v8_4_8_ss_g(), v8_4_8_ts_g(), v8_4_8_vg_g(), v8_5_0_ss_g(), v8_5_0_ts_g(), v8_5_0_vg_g(),
        v8_5_3_ss_g(), v8_5_3_ts_g(), v8_5_3_vg_g(), v8_5_5_ss_g(), v8_5_5_ts_g(), v8_5_5_vg_g(),
        v8_5_7_ss_g(), v8_5_7_ts_g(), v8_5_7_vg_g(), v8_6_0_ss_g(), v8_6_0_ts_g(), v8_6_0_vg_g(),
        v8_6_2_ss_g(), v8_6_2_ts_g(), v8_6_2_vg_g(), v8_6_4_ss_g(), v8_6_4_ts_g(), v8_6_4_vg_g(),
        v8_7_0_ss_g(), v8_7_0_ts_g(), v8_7_0_vg_g(),
    ]
}
