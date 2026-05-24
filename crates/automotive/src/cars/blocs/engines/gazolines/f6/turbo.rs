use crate::components::powertrain::engines::thermals::assemblies::{aspiration::Aspiration, bloc::Bloc, layout::Layout};
use crate::components::powertrain::engines::thermals::parts::culasses::Culasse;
use super::cc;

const L: Layout = Layout::Flat;
const N: u8 = 6;

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

pub fn f6_2_0_ss_g() -> Bloc { make(cc::_2_0::CC, 76.4, 72.8, "SS_g", 10.0, 1.3) }
pub fn f6_2_0_ts_g() -> Bloc { make(cc::_2_0::CC, 76.4, 72.8, "TS_g",  9.5, 1.6) }
pub fn f6_2_0_vg_g() -> Bloc { make(cc::_2_0::CC, 76.4, 72.8, "VG_g",  8.5, 1.8) }
pub fn f6_2_2_ss_g() -> Bloc { make(cc::_2_2::CC, 78.8, 75.0, "SS_g", 10.0, 1.3) }
pub fn f6_2_2_ts_g() -> Bloc { make(cc::_2_2::CC, 78.8, 75.0, "TS_g",  9.5, 1.6) }
pub fn f6_2_2_vg_g() -> Bloc { make(cc::_2_2::CC, 78.8, 75.0, "VG_g",  8.5, 1.8) }
pub fn f6_2_4_ss_g() -> Bloc { make(cc::_2_4::CC, 81.2, 77.3, "SS_g", 10.0, 1.3) }
pub fn f6_2_4_ts_g() -> Bloc { make(cc::_2_4::CC, 81.2, 77.3, "TS_g",  9.5, 1.6) }
pub fn f6_2_4_vg_g() -> Bloc { make(cc::_2_4::CC, 81.2, 77.3, "VG_g",  8.5, 1.8) }
pub fn f6_2_7_ss_g() -> Bloc { make(cc::_2_7::CC, 84.4, 80.4, "SS_g", 10.0, 1.3) }
pub fn f6_2_7_ts_g() -> Bloc { make(cc::_2_7::CC, 84.4, 80.4, "TS_g",  9.5, 1.6) }
pub fn f6_2_7_vg_g() -> Bloc { make(cc::_2_7::CC, 84.4, 80.4, "VG_g",  8.5, 1.8) }
pub fn f6_3_0_ss_g() -> Bloc { make(cc::_3_0::CC, 87.4, 83.2, "SS_g", 10.0, 1.3) }
pub fn f6_3_0_ts_g() -> Bloc { make(cc::_3_0::CC, 87.4, 83.2, "TS_g",  9.5, 1.6) }
pub fn f6_3_0_vg_g() -> Bloc { make(cc::_3_0::CC, 87.4, 83.2, "VG_g",  8.5, 1.8) }
pub fn f6_3_2_ss_g() -> Bloc { make(cc::_3_2::CC, 89.3, 85.0, "SS_g", 10.0, 1.3) }
pub fn f6_3_2_ts_g() -> Bloc { make(cc::_3_2::CC, 89.3, 85.0, "TS_g",  9.5, 1.6) }
pub fn f6_3_2_vg_g() -> Bloc { make(cc::_3_2::CC, 89.3, 85.0, "VG_g",  8.5, 1.8) }
pub fn f6_3_3_ss_g() -> Bloc { make(cc::_3_3::CC, 90.3, 86.0, "SS_g", 10.0, 1.3) }
pub fn f6_3_3_ts_g() -> Bloc { make(cc::_3_3::CC, 90.3, 86.0, "TS_g",  9.5, 1.6) }
pub fn f6_3_3_vg_g() -> Bloc { make(cc::_3_3::CC, 90.3, 86.0, "VG_g",  8.5, 1.8) }
pub fn f6_3_4_ss_g() -> Bloc { make(cc::_3_4::CC, 91.2, 86.9, "SS_g", 10.0, 1.3) }
pub fn f6_3_4_ts_g() -> Bloc { make(cc::_3_4::CC, 91.2, 86.9, "TS_g",  9.5, 1.6) }
pub fn f6_3_4_vg_g() -> Bloc { make(cc::_3_4::CC, 91.2, 86.9, "VG_g",  8.5, 1.8) }
pub fn f6_3_6_ss_g() -> Bloc { make(cc::_3_6::CC, 92.9, 88.5, "SS_g", 10.0, 1.3) }
pub fn f6_3_6_ts_g() -> Bloc { make(cc::_3_6::CC, 92.9, 88.5, "TS_g",  9.5, 1.6) }
pub fn f6_3_6_vg_g() -> Bloc { make(cc::_3_6::CC, 92.9, 88.5, "VG_g",  8.5, 1.8) }
pub fn f6_3_8_ss_g() -> Bloc { make(cc::_3_8::CC, 94.6, 90.1, "SS_g", 10.0, 1.3) }
pub fn f6_3_8_ts_g() -> Bloc { make(cc::_3_8::CC, 94.6, 90.1, "TS_g",  9.5, 1.6) }
pub fn f6_3_8_vg_g() -> Bloc { make(cc::_3_8::CC, 94.6, 90.1, "VG_g",  8.5, 1.8) }
pub fn f6_4_0_ss_g() -> Bloc { make(cc::_4_0::CC, 96.2, 91.6, "SS_g", 10.0, 1.3) }
pub fn f6_4_0_ts_g() -> Bloc { make(cc::_4_0::CC, 96.2, 91.6, "TS_g",  9.5, 1.6) }
pub fn f6_4_0_vg_g() -> Bloc { make(cc::_4_0::CC, 96.2, 91.6, "VG_g",  8.5, 1.8) }

pub fn all() -> Vec<Bloc> {
    vec![
        f6_2_0_ss_g(), f6_2_0_ts_g(), f6_2_0_vg_g(),
        f6_2_2_ss_g(), f6_2_2_ts_g(), f6_2_2_vg_g(),
        f6_2_4_ss_g(), f6_2_4_ts_g(), f6_2_4_vg_g(),
        f6_2_7_ss_g(), f6_2_7_ts_g(), f6_2_7_vg_g(),
        f6_3_0_ss_g(), f6_3_0_ts_g(), f6_3_0_vg_g(),
        f6_3_2_ss_g(), f6_3_2_ts_g(), f6_3_2_vg_g(),
        f6_3_3_ss_g(), f6_3_3_ts_g(), f6_3_3_vg_g(),
        f6_3_4_ss_g(), f6_3_4_ts_g(), f6_3_4_vg_g(),
        f6_3_6_ss_g(), f6_3_6_ts_g(), f6_3_6_vg_g(),
        f6_3_8_ss_g(), f6_3_8_ts_g(), f6_3_8_vg_g(),
        f6_4_0_ss_g(), f6_4_0_ts_g(), f6_4_0_vg_g(),
    ]
}
