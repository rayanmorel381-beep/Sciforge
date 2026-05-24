use crate::components::powertrain::engines::thermals::assemblies::{aspiration::Aspiration, bloc::Bloc, layout::Layout};
use crate::components::powertrain::engines::thermals::parts::culasses::Culasse;
use super::cc;

const L: Layout = Layout::W;
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

pub fn w6_2_8_ss_g() -> Bloc { make(cc::_2_8::CC, 84.9, 82.4, "SS_g", 10.0, 1.3) }
pub fn w6_2_8_ts_g() -> Bloc { make(cc::_2_8::CC, 84.9, 82.4, "TS_g",  9.5, 1.6) }
pub fn w6_2_8_vg_g() -> Bloc { make(cc::_2_8::CC, 84.9, 82.4, "VG_g",  8.5, 1.8) }
pub fn w6_3_2_ss_g() -> Bloc { make(cc::_3_2::CC, 88.8, 86.2, "SS_g", 10.0, 1.3) }
pub fn w6_3_2_ts_g() -> Bloc { make(cc::_3_2::CC, 88.8, 86.2, "TS_g",  9.5, 1.6) }
pub fn w6_3_2_vg_g() -> Bloc { make(cc::_3_2::CC, 88.8, 86.2, "VG_g",  8.5, 1.8) }
pub fn w6_3_5_ss_g() -> Bloc { make(cc::_3_5::CC, 91.5, 88.8, "SS_g", 10.0, 1.3) }
pub fn w6_3_5_ts_g() -> Bloc { make(cc::_3_5::CC, 91.5, 88.8, "TS_g",  9.5, 1.6) }
pub fn w6_3_5_vg_g() -> Bloc { make(cc::_3_5::CC, 91.5, 88.8, "VG_g",  8.5, 1.8) }
pub fn w6_3_6_ss_g() -> Bloc { make(cc::_3_6::CC, 92.3, 89.6, "SS_g", 10.0, 1.3) }
pub fn w6_3_6_ts_g() -> Bloc { make(cc::_3_6::CC, 92.3, 89.6, "TS_g",  9.5, 1.6) }
pub fn w6_3_6_vg_g() -> Bloc { make(cc::_3_6::CC, 92.3, 89.6, "VG_g",  8.5, 1.8) }

pub fn all() -> Vec<Bloc> {
    vec![w6_2_8_ss_g(), w6_2_8_ts_g(), w6_2_8_vg_g(), w6_3_2_ss_g(), w6_3_2_ts_g(), w6_3_2_vg_g(), w6_3_6_ss_g(), w6_3_6_ts_g(), w6_3_6_vg_g()]
}
