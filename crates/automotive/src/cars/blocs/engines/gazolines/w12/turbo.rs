use crate::components::powertrain::engines::thermals::assemblies::{aspiration::Aspiration, bloc::Bloc, layout::Layout};
use crate::components::powertrain::engines::thermals::parts::culasses::Culasse;
use super::cc;

const L: Layout = Layout::W;
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

pub fn w12_5_0_ss_g() -> Bloc { make(cc::_5_0::CC, 83.1, 76.9, "SS_g", 10.0, 1.3) }
pub fn w12_5_0_ts_g() -> Bloc { make(cc::_5_0::CC, 83.1, 76.9, "TS_g",  9.5, 1.6) }
pub fn w12_5_0_vg_g() -> Bloc { make(cc::_5_0::CC, 83.1, 76.9, "VG_g",  8.5, 1.8) }
pub fn w12_6_0_ss_g() -> Bloc { make(cc::_6_0::CC, 88.3, 81.8, "SS_g", 10.0, 1.3) }
pub fn w12_6_0_ts_g() -> Bloc { make(cc::_6_0::CC, 88.3, 81.8, "TS_g",  9.5, 1.6) }
pub fn w12_6_0_vg_g() -> Bloc { make(cc::_6_0::CC, 88.3, 81.8, "VG_g",  8.5, 1.8) }
pub fn w12_6_3_ss_g() -> Bloc { make(cc::_6_3::CC, 89.7, 83.1, "SS_g", 10.0, 1.3) }
pub fn w12_6_3_ts_g() -> Bloc { make(cc::_6_3::CC, 89.7, 83.1, "TS_g",  9.5, 1.6) }
pub fn w12_6_3_vg_g() -> Bloc { make(cc::_6_3::CC, 89.7, 83.1, "VG_g",  8.5, 1.8) }

pub fn all() -> Vec<Bloc> {
    vec![w12_5_0_ss_g(), w12_5_0_ts_g(), w12_5_0_vg_g(), w12_6_0_ss_g(), w12_6_0_ts_g(), w12_6_0_vg_g(), w12_6_3_ss_g(), w12_6_3_ts_g(), w12_6_3_vg_g()]
}
