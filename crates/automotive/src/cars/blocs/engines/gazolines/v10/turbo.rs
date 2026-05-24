use crate::components::powertrain::engines::thermals::assemblies::{aspiration::Aspiration, bloc::Bloc, layout::Layout};
use crate::components::powertrain::engines::thermals::parts::culasses::Culasse;
use super::cc;

const L: Layout = Layout::V;
const N: u8 = 10;

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

pub fn v10_4_0_ss_g() -> Bloc { make(cc::_4_0::CC, 82.4, 74.9, "SS_g", 10.0, 1.3) }
pub fn v10_4_0_ts_g() -> Bloc { make(cc::_4_0::CC, 82.4, 74.9, "TS_g",  9.5, 1.6) }
pub fn v10_4_0_vg_g() -> Bloc { make(cc::_4_0::CC, 82.4, 74.9, "VG_g",  8.5, 1.8) }
pub fn v10_4_2_ss_g() -> Bloc { make(cc::_4_2::CC, 83.8, 76.2, "SS_g", 10.0, 1.3) }
pub fn v10_4_2_ts_g() -> Bloc { make(cc::_4_2::CC, 83.8, 76.2, "TS_g",  9.5, 1.6) }
pub fn v10_4_2_vg_g() -> Bloc { make(cc::_4_2::CC, 83.8, 76.2, "VG_g",  8.5, 1.8) }
pub fn v10_5_0_ss_g() -> Bloc { make(cc::_5_0::CC, 88.8, 80.7, "SS_g", 10.0, 1.3) }
pub fn v10_5_0_ts_g() -> Bloc { make(cc::_5_0::CC, 88.8, 80.7, "TS_g",  9.5, 1.6) }
pub fn v10_5_0_vg_g() -> Bloc { make(cc::_5_0::CC, 88.8, 80.7, "VG_g",  8.5, 1.8) }
pub fn v10_5_2_ss_g() -> Bloc { make(cc::_5_2::CC, 90.0, 81.8, "SS_g", 10.0, 1.3) }
pub fn v10_5_2_ts_g() -> Bloc { make(cc::_5_2::CC, 90.0, 81.8, "TS_g",  9.5, 1.6) }
pub fn v10_5_2_vg_g() -> Bloc { make(cc::_5_2::CC, 90.0, 81.8, "VG_g",  8.5, 1.8) }
pub fn v10_5_7_ss_g() -> Bloc { make(cc::_5_7::CC, 92.8, 84.4, "SS_g", 10.0, 1.3) }
pub fn v10_5_7_ts_g() -> Bloc { make(cc::_5_7::CC, 92.8, 84.4, "TS_g",  9.5, 1.6) }
pub fn v10_5_7_vg_g() -> Bloc { make(cc::_5_7::CC, 92.8, 84.4, "VG_g",  8.5, 1.8) }
pub fn v10_5_5_ss_g() -> Bloc { make(cc::_5_5::CC, 91.7, 83.4, "SS_g", 10.0, 1.3) }
pub fn v10_5_5_ts_g() -> Bloc { make(cc::_5_5::CC, 91.7, 83.4, "TS_g",  9.5, 1.6) }
pub fn v10_5_5_vg_g() -> Bloc { make(cc::_5_5::CC, 91.7, 83.4, "VG_g",  8.5, 1.8) }
pub fn v10_6_0_ss_g() -> Bloc { make(cc::_6_0::CC, 94.4, 85.8, "SS_g", 10.0, 1.3) }
pub fn v10_6_0_ts_g() -> Bloc { make(cc::_6_0::CC, 94.4, 85.8, "TS_g",  9.5, 1.6) }
pub fn v10_6_0_vg_g() -> Bloc { make(cc::_6_0::CC, 94.4, 85.8, "VG_g",  8.5, 1.8) }
pub fn v10_8_3_ss_g() -> Bloc { make(cc::_8_3::CC, 105.1, 95.5, "SS_g", 10.0, 1.3) }
pub fn v10_8_3_ts_g() -> Bloc { make(cc::_8_3::CC, 105.1, 95.5, "TS_g",  9.5, 1.6) }
pub fn v10_8_3_vg_g() -> Bloc { make(cc::_8_3::CC, 105.1, 95.5, "VG_g",  8.5, 1.8) }
pub fn v10_8_4_ss_g() -> Bloc { make(cc::_8_4::CC, 105.6, 96.0, "SS_g", 10.0, 1.3) }
pub fn v10_8_4_ts_g() -> Bloc { make(cc::_8_4::CC, 105.6, 96.0, "TS_g",  9.5, 1.6) }
pub fn v10_8_4_vg_g() -> Bloc { make(cc::_8_4::CC, 105.6, 96.0, "VG_g",  8.5, 1.8) }

pub fn all() -> Vec<Bloc> {
    vec![
        v10_4_0_ss_g(), v10_4_0_ts_g(), v10_4_0_vg_g(),
        v10_4_2_ss_g(), v10_4_2_ts_g(), v10_4_2_vg_g(),
        v10_5_0_ss_g(), v10_5_0_ts_g(), v10_5_0_vg_g(),
        v10_5_2_ss_g(), v10_5_2_ts_g(), v10_5_2_vg_g(),
        v10_5_7_ss_g(), v10_5_7_ts_g(), v10_5_7_vg_g(),
        v10_8_3_ss_g(), v10_8_3_ts_g(), v10_8_3_vg_g(),
        v10_8_4_ss_g(), v10_8_4_ts_g(), v10_8_4_vg_g(),
    ]
}
