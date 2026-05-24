use crate::components::powertrain::engines::thermals::assemblies::{aspiration::Aspiration, bloc::Bloc, layout::Layout};
use crate::components::powertrain::engines::thermals::parts::culasses::Culasse;
use super::cc;

const L: Layout = Layout::Inline;
const N: u8 = 3;

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

pub fn i3_0_6_ss_g() -> Bloc { make(cc::_0_6::CC, 61.6, 67.0, "SS_g", 10.0, 1.3) }
pub fn i3_0_6_ts_g() -> Bloc { make(cc::_0_6::CC, 61.6, 67.0, "TS_g",  9.5, 1.6) }
pub fn i3_0_6_vg_g() -> Bloc { make(cc::_0_6::CC, 61.6, 67.0, "VG_g",  8.5, 1.8) }
pub fn i3_0_7_ss_g() -> Bloc { make(cc::_0_7::CC, 64.9, 70.5, "SS_g", 10.0, 1.3) }
pub fn i3_0_7_ts_g() -> Bloc { make(cc::_0_7::CC, 64.9, 70.5, "TS_g",  9.5, 1.6) }
pub fn i3_0_7_vg_g() -> Bloc { make(cc::_0_7::CC, 64.9, 70.5, "VG_g",  8.5, 1.8) }
pub fn i3_0_8_ss_g() -> Bloc { make(cc::_0_8::CC, 67.9, 73.8, "SS_g", 10.0, 1.3) }
pub fn i3_0_8_ts_g() -> Bloc { make(cc::_0_8::CC, 67.9, 73.8, "TS_g",  9.5, 1.6) }
pub fn i3_0_8_vg_g() -> Bloc { make(cc::_0_8::CC, 67.9, 73.8, "VG_g",  8.5, 1.8) }
pub fn i3_0_9_ss_g() -> Bloc { make(cc::_0_9::CC, 70.6, 76.7, "SS_g", 10.0, 1.3) }
pub fn i3_0_9_ts_g() -> Bloc { make(cc::_0_9::CC, 70.6, 76.7, "TS_g",  9.5, 1.6) }
pub fn i3_0_9_vg_g() -> Bloc { make(cc::_0_9::CC, 70.6, 76.7, "VG_g",  8.5, 1.8) }
pub fn i3_1_0_ss_g() -> Bloc { make(cc::_1_0::CC, 73.1, 79.5, "SS_g", 10.0, 1.3) }
pub fn i3_1_0_ts_g() -> Bloc { make(cc::_1_0::CC, 73.1, 79.5, "TS_g",  9.5, 1.6) }
pub fn i3_1_0_vg_g() -> Bloc { make(cc::_1_0::CC, 73.1, 79.5, "VG_g",  8.5, 1.8) }
pub fn i3_1_1_ss_g() -> Bloc { make(cc::_1_1::CC, 75.4, 82.0, "SS_g", 10.0, 1.3) }
pub fn i3_1_1_ts_g() -> Bloc { make(cc::_1_1::CC, 75.4, 82.0, "TS_g",  9.5, 1.6) }
pub fn i3_1_1_vg_g() -> Bloc { make(cc::_1_1::CC, 75.4, 82.0, "VG_g",  8.5, 1.8) }
pub fn i3_1_2_ss_g() -> Bloc { make(cc::_1_2::CC, 77.7, 84.5, "SS_g", 10.0, 1.3) }
pub fn i3_1_2_ts_g() -> Bloc { make(cc::_1_2::CC, 77.7, 84.5, "TS_g",  9.5, 1.6) }
pub fn i3_1_2_vg_g() -> Bloc { make(cc::_1_2::CC, 77.7, 84.5, "VG_g",  8.5, 1.8) }
pub fn i3_1_3_ss_g() -> Bloc { make(cc::_1_3::CC, 79.8, 86.7, "SS_g", 10.0, 1.3) }
pub fn i3_1_3_ts_g() -> Bloc { make(cc::_1_3::CC, 79.8, 86.7, "TS_g",  9.5, 1.6) }
pub fn i3_1_3_vg_g() -> Bloc { make(cc::_1_3::CC, 79.8, 86.7, "VG_g",  8.5, 1.8) }
pub fn i3_1_4_ss_g() -> Bloc { make(cc::_1_4::CC, 81.8, 88.9, "SS_g", 10.0, 1.3) }
pub fn i3_1_4_ts_g() -> Bloc { make(cc::_1_4::CC, 81.8, 88.9, "TS_g",  9.5, 1.6) }
pub fn i3_1_4_vg_g() -> Bloc { make(cc::_1_4::CC, 81.8, 88.9, "VG_g",  8.5, 1.8) }
pub fn i3_1_5_ss_g() -> Bloc { make(cc::_1_5::CC, 83.7, 91.0, "SS_g", 10.0, 1.3) }
pub fn i3_1_5_ts_g() -> Bloc { make(cc::_1_5::CC, 83.7, 91.0, "TS_g",  9.5, 1.6) }
pub fn i3_1_5_vg_g() -> Bloc { make(cc::_1_5::CC, 83.7, 91.0, "VG_g",  8.5, 1.8) }

pub fn all() -> Vec<Bloc> {
    vec![
        i3_0_6_ss_g(), i3_0_6_ts_g(), i3_0_6_vg_g(),
        i3_0_7_ss_g(), i3_0_7_ts_g(), i3_0_7_vg_g(),
        i3_0_8_ss_g(), i3_0_8_ts_g(), i3_0_8_vg_g(),
        i3_0_9_ss_g(), i3_0_9_ts_g(), i3_0_9_vg_g(),
        i3_1_0_ss_g(), i3_1_0_ts_g(), i3_1_0_vg_g(),
        i3_1_1_ss_g(), i3_1_1_ts_g(), i3_1_1_vg_g(),
        i3_1_2_ss_g(), i3_1_2_ts_g(), i3_1_2_vg_g(),
        i3_1_3_ss_g(), i3_1_3_ts_g(), i3_1_3_vg_g(),
        i3_1_4_ss_g(), i3_1_4_ts_g(), i3_1_4_vg_g(),
        i3_1_5_ss_g(), i3_1_5_ts_g(), i3_1_5_vg_g(),
    ]
}
