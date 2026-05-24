use crate::components::powertrain::engines::thermals::assemblies::{aspiration::Aspiration, bloc::Bloc, layout::Layout};
use crate::components::powertrain::engines::thermals::parts::culasses::Culasse;
use super::cc;

const L: Layout = Layout::Inline;
const N: u8 = 5;

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

pub fn i5_2_0_ss_g() -> Bloc { make(cc::_2_0::CC, 77.7, 84.5, "SS_g", 10.0, 1.3) }
pub fn i5_2_0_ts_g() -> Bloc { make(cc::_2_0::CC, 77.7, 84.5, "TS_g",  9.5, 1.6) }
pub fn i5_2_0_vg_g() -> Bloc { make(cc::_2_0::CC, 77.7, 84.5, "VG_g",  8.5, 1.8) }
pub fn i5_2_1_ss_g() -> Bloc { make(cc::_2_1::CC, 78.9, 85.8, "SS_g", 10.0, 1.3) }
pub fn i5_2_1_ts_g() -> Bloc { make(cc::_2_1::CC, 78.9, 85.8, "TS_g",  9.5, 1.6) }
pub fn i5_2_1_vg_g() -> Bloc { make(cc::_2_1::CC, 78.9, 85.8, "VG_g",  8.5, 1.8) }
pub fn i5_2_2_ss_g() -> Bloc { make(cc::_2_2::CC, 80.2, 87.2, "SS_g", 10.0, 1.3) }
pub fn i5_2_2_ts_g() -> Bloc { make(cc::_2_2::CC, 80.2, 87.2, "TS_g",  9.5, 1.6) }
pub fn i5_2_2_vg_g() -> Bloc { make(cc::_2_2::CC, 80.2, 87.2, "VG_g",  8.5, 1.8) }
pub fn i5_2_3_ss_g() -> Bloc { make(cc::_2_3::CC, 81.4, 88.5, "SS_g", 10.0, 1.3) }
pub fn i5_2_3_ts_g() -> Bloc { make(cc::_2_3::CC, 81.4, 88.5, "TS_g",  9.5, 1.6) }
pub fn i5_2_3_vg_g() -> Bloc { make(cc::_2_3::CC, 81.4, 88.5, "VG_g",  8.5, 1.8) }
pub fn i5_2_4_ss_g() -> Bloc { make(cc::_2_4::CC, 82.5, 89.7, "SS_g", 10.0, 1.3) }
pub fn i5_2_4_ts_g() -> Bloc { make(cc::_2_4::CC, 82.5, 89.7, "TS_g",  9.5, 1.6) }
pub fn i5_2_4_vg_g() -> Bloc { make(cc::_2_4::CC, 82.5, 89.7, "VG_g",  8.5, 1.8) }
pub fn i5_2_5_ss_g() -> Bloc { make(cc::_2_5::CC, 83.7, 91.0, "SS_g", 10.0, 1.3) }
pub fn i5_2_5_ts_g() -> Bloc { make(cc::_2_5::CC, 83.7, 91.0, "TS_g",  9.5, 1.6) }
pub fn i5_2_5_vg_g() -> Bloc { make(cc::_2_5::CC, 83.7, 91.0, "VG_g",  8.5, 1.8) }

pub fn all() -> Vec<Bloc> {
    vec![
        i5_2_0_ss_g(), i5_2_0_ts_g(), i5_2_0_vg_g(),
        i5_2_1_ss_g(), i5_2_1_ts_g(), i5_2_1_vg_g(),
        i5_2_2_ss_g(), i5_2_2_ts_g(), i5_2_2_vg_g(),
        i5_2_3_ss_g(), i5_2_3_ts_g(), i5_2_3_vg_g(),
        i5_2_4_ss_g(), i5_2_4_ts_g(), i5_2_4_vg_g(),
        i5_2_5_ss_g(), i5_2_5_ts_g(), i5_2_5_vg_g(),
    ]
}
