use crate::components::powertrain::engines::thermals::assemblies::{aspiration::Aspiration, bloc::Bloc, layout::Layout};
use crate::components::powertrain::engines::thermals::parts::culasses::Culasse;
use super::cc;

const L: Layout = Layout::Inline;
const N: u8 = 4;

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

pub fn i4_1_0_ss_g() -> Bloc { make(cc::_1_0::CC, 66.4, 72.2, "SS_g", 10.0, 1.3) }
pub fn i4_1_0_ts_g() -> Bloc { make(cc::_1_0::CC, 66.4, 72.2, "TS_g",  9.5, 1.6) }
pub fn i4_1_0_vg_g() -> Bloc { make(cc::_1_0::CC, 66.4, 72.2, "VG_g",  8.5, 1.8) }
pub fn i4_1_1_ss_g() -> Bloc { make(cc::_1_1::CC, 68.6, 74.6, "SS_g", 10.0, 1.3) }
pub fn i4_1_1_ts_g() -> Bloc { make(cc::_1_1::CC, 68.6, 74.6, "TS_g",  9.5, 1.6) }
pub fn i4_1_1_vg_g() -> Bloc { make(cc::_1_1::CC, 68.6, 74.6, "VG_g",  8.5, 1.8) }
pub fn i4_1_2_ss_g() -> Bloc { make(cc::_1_2::CC, 70.6, 76.7, "SS_g", 10.0, 1.3) }
pub fn i4_1_2_ts_g() -> Bloc { make(cc::_1_2::CC, 70.6, 76.7, "TS_g",  9.5, 1.6) }
pub fn i4_1_2_vg_g() -> Bloc { make(cc::_1_2::CC, 70.6, 76.7, "VG_g",  8.5, 1.8) }
pub fn i4_1_3_ss_g() -> Bloc { make(cc::_1_3::CC, 72.5, 78.8, "SS_g", 10.0, 1.3) }
pub fn i4_1_3_ts_g() -> Bloc { make(cc::_1_3::CC, 72.5, 78.8, "TS_g",  9.5, 1.6) }
pub fn i4_1_3_vg_g() -> Bloc { make(cc::_1_3::CC, 72.5, 78.8, "VG_g",  8.5, 1.8) }
pub fn i4_1_4_ss_g() -> Bloc { make(cc::_1_4::CC, 74.3, 80.8, "SS_g", 10.0, 1.3) }
pub fn i4_1_4_ts_g() -> Bloc { make(cc::_1_4::CC, 74.3, 80.8, "TS_g",  9.5, 1.6) }
pub fn i4_1_4_vg_g() -> Bloc { make(cc::_1_4::CC, 74.3, 80.8, "VG_g",  8.5, 1.8) }
pub fn i4_1_5_ss_g() -> Bloc { make(cc::_1_5::CC, 76.0, 82.6, "SS_g", 10.0, 1.3) }
pub fn i4_1_5_ts_g() -> Bloc { make(cc::_1_5::CC, 76.0, 82.6, "TS_g",  9.5, 1.6) }
pub fn i4_1_5_vg_g() -> Bloc { make(cc::_1_5::CC, 76.0, 82.6, "VG_g",  8.5, 1.8) }
pub fn i4_1_6_ss_g() -> Bloc { make(cc::_1_6::CC, 77.7, 84.5, "SS_g", 10.0, 1.3) }
pub fn i4_1_6_ts_g() -> Bloc { make(cc::_1_6::CC, 77.7, 84.5, "TS_g",  9.5, 1.6) }
pub fn i4_1_6_vg_g() -> Bloc { make(cc::_1_6::CC, 77.7, 84.5, "VG_g",  8.5, 1.8) }
pub fn i4_1_7_ss_g() -> Bloc { make(cc::_1_7::CC, 79.3, 86.2, "SS_g", 10.0, 1.3) }
pub fn i4_1_7_ts_g() -> Bloc { make(cc::_1_7::CC, 79.3, 86.2, "TS_g",  9.5, 1.6) }
pub fn i4_1_7_vg_g() -> Bloc { make(cc::_1_7::CC, 79.3, 86.2, "VG_g",  8.5, 1.8) }
pub fn i4_1_8_ss_g() -> Bloc { make(cc::_1_8::CC, 80.8, 87.8, "SS_g", 10.0, 1.3) }
pub fn i4_1_8_ts_g() -> Bloc { make(cc::_1_8::CC, 80.8, 87.8, "TS_g",  9.5, 1.6) }
pub fn i4_1_8_vg_g() -> Bloc { make(cc::_1_8::CC, 80.8, 87.8, "VG_g",  8.5, 1.8) }
pub fn i4_1_9_ss_g() -> Bloc { make(cc::_1_9::CC, 82.2, 89.3, "SS_g", 10.0, 1.3) }
pub fn i4_1_9_ts_g() -> Bloc { make(cc::_1_9::CC, 82.2, 89.3, "TS_g",  9.5, 1.6) }
pub fn i4_1_9_vg_g() -> Bloc { make(cc::_1_9::CC, 82.2, 89.3, "VG_g",  8.5, 1.8) }
pub fn i4_2_0_ss_g() -> Bloc { make(cc::_2_0::CC, 83.7, 91.0, "SS_g", 10.0, 1.3) }
pub fn i4_2_0_ts_g() -> Bloc { make(cc::_2_0::CC, 83.7, 91.0, "TS_g",  9.5, 1.6) }
pub fn i4_2_0_vg_g() -> Bloc { make(cc::_2_0::CC, 83.7, 91.0, "VG_g",  8.5, 1.8) }
pub fn i4_2_1_ss_g() -> Bloc { make(cc::_2_1::CC, 85.0, 92.4, "SS_g", 10.0, 1.3) }
pub fn i4_2_1_ts_g() -> Bloc { make(cc::_2_1::CC, 85.0, 92.4, "TS_g",  9.5, 1.6) }
pub fn i4_2_1_vg_g() -> Bloc { make(cc::_2_1::CC, 85.0, 92.4, "VG_g",  8.5, 1.8) }
pub fn i4_2_2_ss_g() -> Bloc { make(cc::_2_2::CC, 86.4, 93.9, "SS_g", 10.0, 1.3) }
pub fn i4_2_2_ts_g() -> Bloc { make(cc::_2_2::CC, 86.4, 93.9, "TS_g",  9.5, 1.6) }
pub fn i4_2_2_vg_g() -> Bloc { make(cc::_2_2::CC, 86.4, 93.9, "VG_g",  8.5, 1.8) }
pub fn i4_2_3_ss_g() -> Bloc { make(cc::_2_3::CC, 87.7, 95.3, "SS_g", 10.0, 1.3) }
pub fn i4_2_3_ts_g() -> Bloc { make(cc::_2_3::CC, 87.7, 95.3, "TS_g",  9.5, 1.6) }
pub fn i4_2_3_vg_g() -> Bloc { make(cc::_2_3::CC, 87.7, 95.3, "VG_g",  8.5, 1.8) }
pub fn i4_2_4_ss_g() -> Bloc { make(cc::_2_4::CC, 88.9, 96.6, "SS_g", 10.0, 1.3) }
pub fn i4_2_4_ts_g() -> Bloc { make(cc::_2_4::CC, 88.9, 96.6, "TS_g",  9.5, 1.6) }
pub fn i4_2_4_vg_g() -> Bloc { make(cc::_2_4::CC, 88.9, 96.6, "VG_g",  8.5, 1.8) }

pub fn all() -> Vec<Bloc> {
    vec![
        i4_1_0_ss_g(), i4_1_0_ts_g(), i4_1_0_vg_g(),
        i4_1_1_ss_g(), i4_1_1_ts_g(), i4_1_1_vg_g(),
        i4_1_2_ss_g(), i4_1_2_ts_g(), i4_1_2_vg_g(),
        i4_1_3_ss_g(), i4_1_3_ts_g(), i4_1_3_vg_g(),
        i4_1_4_ss_g(), i4_1_4_ts_g(), i4_1_4_vg_g(),
        i4_1_5_ss_g(), i4_1_5_ts_g(), i4_1_5_vg_g(),
        i4_1_6_ss_g(), i4_1_6_ts_g(), i4_1_6_vg_g(),
        i4_1_7_ss_g(), i4_1_7_ts_g(), i4_1_7_vg_g(),
        i4_1_8_ss_g(), i4_1_8_ts_g(), i4_1_8_vg_g(),
        i4_1_9_ss_g(), i4_1_9_ts_g(), i4_1_9_vg_g(),
        i4_2_0_ss_g(), i4_2_0_ts_g(), i4_2_0_vg_g(),
        i4_2_1_ss_g(), i4_2_1_ts_g(), i4_2_1_vg_g(),
        i4_2_2_ss_g(), i4_2_2_ts_g(), i4_2_2_vg_g(),
        i4_2_3_ss_g(), i4_2_3_ts_g(), i4_2_3_vg_g(),
        i4_2_4_ss_g(), i4_2_4_ts_g(), i4_2_4_vg_g(),
    ]
}
