use crate::components::powertrain::engines::thermals::assemblies::{aspiration::Aspiration, bloc::Bloc, layout::Layout};
use crate::components::powertrain::engines::thermals::parts::culasses::Culasse;
use super::cc;

const L: Layout = Layout::Inline;
const N: u8 = 1;

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

pub fn i1_0_1_ss_g() -> Bloc { make(cc::_0_1::CC, 47.7, 56.1, "SS_g", 10.0, 1.3) }
pub fn i1_0_1_ts_g() -> Bloc { make(cc::_0_1::CC, 47.7, 56.1, "TS_g",  9.5, 1.6) }
pub fn i1_0_1_vg_g() -> Bloc { make(cc::_0_1::CC, 47.7, 56.1, "VG_g",  8.5, 1.8) }
pub fn i1_0_2_ss_g() -> Bloc { make(cc::_0_2::CC, 60.0, 70.6, "SS_g", 10.0, 1.3) }
pub fn i1_0_2_ts_g() -> Bloc { make(cc::_0_2::CC, 60.0, 70.6, "TS_g",  9.5, 1.6) }
pub fn i1_0_2_vg_g() -> Bloc { make(cc::_0_2::CC, 60.0, 70.6, "VG_g",  8.5, 1.8) }
pub fn i1_0_3_ss_g() -> Bloc { make(cc::_0_3::CC, 68.7, 80.8, "SS_g", 10.0, 1.3) }
pub fn i1_0_3_ts_g() -> Bloc { make(cc::_0_3::CC, 68.7, 80.8, "TS_g",  9.5, 1.6) }
pub fn i1_0_3_vg_g() -> Bloc { make(cc::_0_3::CC, 68.7, 80.8, "VG_g",  8.5, 1.8) }
pub fn i1_0_4_ss_g() -> Bloc { make(cc::_0_4::CC, 75.6, 88.9, "SS_g", 10.0, 1.3) }
pub fn i1_0_4_ts_g() -> Bloc { make(cc::_0_4::CC, 75.6, 88.9, "TS_g",  9.5, 1.6) }
pub fn i1_0_4_vg_g() -> Bloc { make(cc::_0_4::CC, 75.6, 88.9, "VG_g",  8.5, 1.8) }
pub fn i1_0_5_ss_g() -> Bloc { make(cc::_0_5::CC, 81.5, 95.9, "SS_g", 10.0, 1.3) }
pub fn i1_0_5_ts_g() -> Bloc { make(cc::_0_5::CC, 81.5, 95.9, "TS_g",  9.5, 1.6) }
pub fn i1_0_5_vg_g() -> Bloc { make(cc::_0_5::CC, 81.5, 95.9, "VG_g",  8.5, 1.8) }
pub fn i1_0_6_ss_g() -> Bloc { make(cc::_0_6::CC, 86.6, 101.9, "SS_g", 10.0, 1.3) }
pub fn i1_0_6_ts_g() -> Bloc { make(cc::_0_6::CC, 86.6, 101.9, "TS_g",  9.5, 1.6) }
pub fn i1_0_6_vg_g() -> Bloc { make(cc::_0_6::CC, 86.6, 101.9, "VG_g",  8.5, 1.8) }
pub fn i1_0_7_ss_g() -> Bloc { make(cc::_0_7::CC, 91.2, 107.3, "SS_g", 10.0, 1.3) }
pub fn i1_0_7_ts_g() -> Bloc { make(cc::_0_7::CC, 91.2, 107.3, "TS_g",  9.5, 1.6) }
pub fn i1_0_7_vg_g() -> Bloc { make(cc::_0_7::CC, 91.2, 107.3, "VG_g",  8.5, 1.8) }
pub fn i1_0_8_ss_g() -> Bloc { make(cc::_0_8::CC, 95.3, 112.1, "SS_g", 10.0, 1.3) }
pub fn i1_0_8_ts_g() -> Bloc { make(cc::_0_8::CC, 95.3, 112.1, "TS_g",  9.5, 1.6) }
pub fn i1_0_8_vg_g() -> Bloc { make(cc::_0_8::CC, 95.3, 112.1, "VG_g",  8.5, 1.8) }

pub fn all() -> Vec<Bloc> {
    vec![
        i1_0_1_ss_g(), i1_0_1_ts_g(), i1_0_1_vg_g(),
        i1_0_2_ss_g(), i1_0_2_ts_g(), i1_0_2_vg_g(),
        i1_0_3_ss_g(), i1_0_3_ts_g(), i1_0_3_vg_g(),
        i1_0_4_ss_g(), i1_0_4_ts_g(), i1_0_4_vg_g(),
        i1_0_5_ss_g(), i1_0_5_ts_g(), i1_0_5_vg_g(),
        i1_0_6_ss_g(), i1_0_6_ts_g(), i1_0_6_vg_g(),
        i1_0_7_ss_g(), i1_0_7_ts_g(), i1_0_7_vg_g(),
        i1_0_8_ss_g(), i1_0_8_ts_g(), i1_0_8_vg_g(),
    ]
}
