use crate::components::powertrain::engines::thermals::assemblies::{aspiration::Aspiration, bloc::Bloc, layout::Layout};
use crate::components::powertrain::engines::thermals::parts::culasses::Culasse;
use super::cc;

const L: Layout = Layout::W;
const N: u8 = 12;

fn make(cc_val: u32, bore: f64, stroke: f64, variant: &'static str, boost: f64) -> Bloc {
    Bloc {
        layout: L, cylinders: N,
        displacement_cc: cc_val, bore_mm: bore, stroke_mm: stroke,
        aspiration: Aspiration::Supercharged, variant,
        compression_ratio: 9.2, max_boost_bar: boost,
        conrod_ratio_lambda: 0.30,
        design_safety_factor: 2.5,
        head_variants: vec![Culasse::ohv_2v_1p_wedge(N), Culasse::sohc_2v_1p_wedge(N), Culasse::dohc_4v_1p_pent_roof(N), Culasse::dohc_4v_2p_hemi(N), Culasse::dohc_5v_1p(N)],
    }
}

pub fn w12_5_0_rt_g() -> Bloc { make(cc::_5_0::CC, 83.1, 76.9, "RT_g", 0.8) }
pub fn w12_5_0_tw_g() -> Bloc { make(cc::_5_0::CC, 83.1, 76.9, "TW_g", 0.9) }
pub fn w12_5_0_cf_g() -> Bloc { make(cc::_5_0::CC, 83.1, 76.9, "CF_g", 0.7) }
pub fn w12_6_0_rt_g() -> Bloc { make(cc::_6_0::CC, 88.3, 81.8, "RT_g", 0.8) }
pub fn w12_6_0_tw_g() -> Bloc { make(cc::_6_0::CC, 88.3, 81.8, "TW_g", 0.9) }
pub fn w12_6_0_cf_g() -> Bloc { make(cc::_6_0::CC, 88.3, 81.8, "CF_g", 0.7) }
pub fn w12_6_3_rt_g() -> Bloc { make(cc::_6_3::CC, 89.7, 83.1, "RT_g", 0.8) }
pub fn w12_6_3_tw_g() -> Bloc { make(cc::_6_3::CC, 89.7, 83.1, "TW_g", 0.9) }
pub fn w12_6_3_cf_g() -> Bloc { make(cc::_6_3::CC, 89.7, 83.1, "CF_g", 0.7) }

pub fn all() -> Vec<Bloc> {
    vec![w12_5_0_rt_g(), w12_5_0_tw_g(), w12_5_0_cf_g(), w12_6_0_rt_g(), w12_6_0_tw_g(), w12_6_0_cf_g(), w12_6_3_rt_g(), w12_6_3_tw_g(), w12_6_3_cf_g()]
}
